module social_contracts::subscription {
    use mys::{clock::Clock, coin::Coin, mys::mys, transfer, object, tx_context};

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
            owner: ctx.sender(),
        }
    }

    entry fun create_service_entry(fee: u64, ttl: u64, ctx: &mut TxContext) {
        transfer::share_object(create_service(fee, ttl, ctx));
    }

    public fun subscribe(
        fee: Coin<mys>,
        service: &Service,
        c: &Clock,
        ctx: &mut TxContext,
    ): Subscription {
        assert!(fee.value() == service.fee, EInvalidFee);
        transfer::public_transfer(fee, service.owner);
        Subscription {
            id: object::new(ctx),
            service_id: object::id(service),
            created_at: c.timestamp_ms(),
        }
    }

    public fun transfer(sub: Subscription, to: address) {
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
            return false;
        };
        if (c.timestamp_ms() > sub.created_at + service.ttl) {
            return false;
        };
        let namespace = service.id.to_bytes();
        let mut i = 0;
        if (namespace.length() > id.length()) {
            return false;
        };
        while (i < namespace.length()) {
            if (namespace[i] != id[i]) {
                return false;
            };
            i = i + 1;
        };
        true
    }

    entry fun seal_approve(id: vector<u8>, sub: &Subscription, service: &Service, c: &Clock) {
        assert!(check_policy(id, sub, service, c), ENoAccess);
    }
}
