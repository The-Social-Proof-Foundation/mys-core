module social_contracts::subscription {
    use mys::clock::{Self, Clock};
    use mys::coin::{Self, Coin};
    use mys::mys::MYS;

    const EInvalidFee: u64 = 12;
    const ENoAccess: u64 = 77;

    public struct Service has key {
        id: UID,
        fee: u64,
        ttl: u64,
        owner: address,
    }

    public struct Subscription has key {
        id: UID,
        service_id: ID,
        created_at: u64,
    }

    public fun create_service(fee: u64, ttl: u64, ctx: &mut TxContext): Service {
        Service {
            id: object::new(ctx),
            fee,
            ttl,
            owner: tx_context::sender(ctx),
        }
    }

    entry fun create_service_entry(fee: u64, ttl: u64, ctx: &mut TxContext) {
        transfer::share_object(create_service(fee, ttl, ctx));
    }

    public fun subscribe(
        fee: Coin<MYS>,
        service: &Service,
        c: &Clock,
        ctx: &mut TxContext,
    ): Subscription {
        assert!(coin::value(&fee) == service.fee, EInvalidFee);
        transfer::public_transfer(fee, service.owner);
        Subscription {
            id: object::new(ctx),
            service_id: object::id(service),
            created_at: clock::timestamp_ms(c),
        }
    }

    public fun transfer_subscription(sub: Subscription, to: address) {
        transfer::transfer(sub, to);
    }

    #[test_only]
    public fun destroy_for_testing(ser: Service, sub: Subscription) {
        let Service { id, .. } = ser;
        object::delete(id);
        let Subscription { id, .. } = sub;
        object::delete(id);
    }

    fun check_policy(id: vector<u8>, sub: &Subscription, service: &Service, c: &Clock): bool {
        if (object::id(service) != sub.service_id) {
            return false
        };
        if (clock::timestamp_ms(c) > sub.created_at + service.ttl) {
            return false
        };
        let namespace = object::uid_to_bytes(&service.id);
        let mut i = 0;
        if (vector::length(&namespace) > vector::length(&id)) {
            return false
        };
        while (i < vector::length(&namespace)) {
            if (*vector::borrow(&namespace, i) != *vector::borrow(&id, i)) {
                return false
            };
            i = i + 1;
        };
        true
    }

    public entry fun seal_approve(id: vector<u8>, sub: &Subscription, service: &Service, c: &Clock) {
        assert!(check_policy(id, sub, service, c), ENoAccess);
    }
}
