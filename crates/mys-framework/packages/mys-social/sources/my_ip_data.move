module social_contracts::my_ip_data {
    use std::string::String;
    use mys::{table::{Self, Table}};
    use mys::coin::Coin;
    use mys::mys::MYS;
    use mys::clock::Clock;
    use seal::bf_hmac_encryption;
    use social_contracts::subscription;

    /// Error codes
    const EPriceMismatch: u64 = 2;
    const ENoSubscriptionService: u64 = 4;

    /// Basic categorizable data that can be licensed
    public struct MyIPData has key, store {
        id: UID,
        owner: address,
        data_type: String,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: u64,
        tags: vector<String>,
        encrypted_uri: Option<vector<u8>>,
        encryption_id: Option<vector<u8>>,
        service_id: Option<ID>,
        price: Option<u64>,
        royalty_split: Table<address, u64>,
        version: u64,
        purchasers: Table<address, bool>,
    }

    public fun create(
        owner: address,
        data_type: String,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: u64,
        tags: vector<String>,
        encrypted_uri: Option<vector<u8>>,
        enc_id: Option<vector<u8>>,
        service_id: Option<ID>,
        price: Option<u64>,
        ctx: &mut TxContext,
    ): MyIPData {
        MyIPData {
            id: object::new(ctx),
            owner,
            data_type,
            platform_id,
            timestamp_start,
            timestamp_end,
            tags,
            encrypted_uri,
            encryption_id: enc_id,
            service_id,
            price,
            royalty_split: table::new(ctx),
            version: 1,
            purchasers: table::new(ctx),
        }
    }

    public entry fun purchase(
        data: &mut MyIPData,
        payment: Coin<MYS>,
        ctx: &mut TxContext,
    ) {
        assert!(option::is_some(&data.price), EPriceMismatch);
        let price = *option::borrow(&data.price);
        assert!(payment.value() == price, EPriceMismatch);
        transfer::public_transfer(payment, data.owner);
        table::add(&mut data.purchasers, tx_context::sender(ctx), true);
    }

    public fun has_access(data: &MyIPData, addr: address): bool {
        table::contains(&data.purchasers, addr)
    }

    /// Decrypt the encrypted URI for a viewer
    public fun decrypt_uri_for(
        data: &MyIPData,
        viewer: address,
        sub: &subscription::Subscription,
        service: &subscription::Service,
        c: &Clock,
        keys: &vector<bf_hmac_encryption::VerifiedDerivedKey>,
        pks: &vector<bf_hmac_encryption::PublicKey>,
    ): Option<vector<u8>> {
        if (!option::is_some(&data.encrypted_uri)) return option::none();
        if (table::contains(&data.purchasers, viewer)) {
            let obj = bf_hmac_encryption::parse_encrypted_object(
                *option::borrow(&data.encrypted_uri)
            );
            return bf_hmac_encryption::decrypt(&obj, keys, pks)
        };
        let sid = *option::borrow(&data.service_id);
        assert!(sid == object::id(service), ENoSubscriptionService);
        let eid = *option::borrow(&data.encryption_id);
        subscription::seal_approve(eid, sub, service, c);
        let obj = bf_hmac_encryption::parse_encrypted_object(
            *option::borrow(&data.encrypted_uri)
        );
        bf_hmac_encryption::decrypt(&obj, keys, pks)
    }
}
