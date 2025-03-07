module mys_framework::my_ip {
    use std::string::{Self, String};
    use std::vector;
    use std::option::{Self, Option};
    
    use mys_framework::object::{Self, UID};
    use mys_framework::tx_context::{Self, TxContext};
    use mys_framework::event;
    use mys_framework::transfer;
    use mys_framework::table::{Self, Table};
    use mys_framework::url::{Self, Url};
    use mys_framework::profile::{Self, Profile};
    use mys_framework::clock::{Self, Clock};
    use mys_framework::display;
    use mys_framework::coin::{Self, Coin};
    use mys_framework::balance::{Self, Balance};
    use mys_framework::mys::MYS;
    
    /// Error codes
    const EUnauthorized: u64 = 0;
    const EInvalidIPType: u64 = 1;
    const EDuplicateLicense: u64 = 2;
    const ELicenseNotFound: u64 = 3;
    const ELicenseNotActive: u64 = 4;
    const ELicenseExpired: u64 = 5;
    const EInvalidPayment: u64 = 6;
    const EDisputeExists: u64 = 7;
    const EDisputeNotFound: u64 = 8;
    const EDisputeNotActive: u64 = 9;
    const EInvalidResolution: u64 = 10;
    
    /// IP types supported by the platform
    const IP_TYPE_CONTENT: u8 = 0;    // Blog posts, articles, etc.
    const IP_TYPE_IMAGE: u8 = 1;      // Images, photos, illustrations
    const IP_TYPE_AUDIO: u8 = 2;      // Music, podcasts, sound effects
    const IP_TYPE_VIDEO: u8 = 3;      // Videos, animations, footage
    const IP_TYPE_CODE: u8 = 4;       // Software, algorithms, etc.
    const IP_TYPE_DESIGN: u8 = 5;     // UI/UX designs, graphics
    const IP_TYPE_OTHER: u8 = 255;    // Other types not categorized
    
    /// License types
    const LICENSE_TYPE_EXCLUSIVE: u8 = 0;      // Only one licensee can use the IP at a time
    const LICENSE_TYPE_NON_EXCLUSIVE: u8 = 1;  // Multiple licensees can use the IP simultaneously
    const LICENSE_TYPE_ATTRIBUTION: u8 = 2;    // Free to use with attribution required
    const LICENSE_TYPE_COMMERCIAL: u8 = 3;     // For commercial use
    const LICENSE_TYPE_PERSONAL: u8 = 4;       // For personal use only
    
    /// License termination status
    const LICENSE_ACTIVE: u8 = 0;
    const LICENSE_EXPIRED: u8 = 1;
    const LICENSE_TERMINATED: u8 = 2;
    const LICENSE_DISPUTED: u8 = 3;
    
    /// Dispute status
    const DISPUTE_ACTIVE: u8 = 0;
    const DISPUTE_RESOLVED: u8 = 1;
    const DISPUTE_REJECTED: u8 = 2;
    
    /// Resolution outcome
    const RESOLUTION_ORIGINAL_CREATOR: u8 = 0;   // Original creator wins dispute
    const RESOLUTION_CHALLENGER: u8 = 1;         // Challenger wins dispute
    const RESOLUTION_SHARED: u8 = 2;             // IP ownership is shared
    
    /// MyIP object representing intellectual property
    struct MyIP has key, store {
        id: UID,
        /// Creator's profile ID
        creator: address,
        /// Title of the IP
        title: String,
        /// Description of the IP
        description: String,
        /// Type of IP (content, image, audio, etc.)
        ip_type: u8,
        /// URL to the IP content (if applicable)
        content_url: Option<Url>,
        /// Hash of original content for verification
        content_hash: vector<u8>,
        /// Optional metadata (can include creation details)
        metadata: String,
        /// Proof of Creativity ID (optional, links to PoC)
        poc_id: Option<address>,
        /// Creation timestamp
        created_at: u64,
        /// Flag indicating if transferable
        transferable: bool,
        /// Royalty percentage for secondary sales (in basis points, 100 = 1%)
        royalty_basis_points: u16,
        /// Registered in countries (ISO country codes)
        registered_countries: vector<String>,
        /// IPO - If this IP has been offered as an IPO token
        ipo_tokenized: bool,
    }
    
    /// License granted for an IP
    struct License has key, store {
        id: UID,
        /// ID of the IP this license is for
        ip_id: address,
        /// Type of license
        license_type: u8,
        /// License terms encoded as a string
        terms: String,
        /// User who holds this license
        licensee: address,
        /// When the license was granted
        granted_at: u64,
        /// Expiration timestamp (0 for perpetual)
        expires_at: u64,
        /// License status (active, expired, terminated, disputed)
        status: u8,
        /// Payment received for this license
        payment_amount: u64,
    }
    
    /// Dispute for an IP
    struct IPDispute has key {
        id: UID,
        /// ID of the disputed IP
        ip_id: address,
        /// User challenging the IP ownership
        challenger: address,
        /// Original creator of the IP
        original_creator: address,
        /// Reason for dispute
        reason: String,
        /// Evidence provided (can be hash or URL)
        evidence: String,
        /// When the dispute was created
        created_at: u64,
        /// Status of the dispute
        status: u8,
        /// Resolution outcome (if resolved)
        resolution: u8,
        /// Resolver address (admin or oracle)
        resolver: Option<address>,
        /// Resolution notes
        resolution_notes: String,
    }
    
    /// Registry to track all IP assets
    struct IPRegistry has key {
        id: UID,
        /// Table mapping creator address to their IP IDs
        creator_ips: Table<address, vector<address>>,
        /// Table mapping IP type to IP IDs
        ip_by_type: Table<u8, vector<address>>,
        /// Total number of registered IPs
        total_ips: u64,
        /// Admin address
        admin: address,
    }
    
    /// Events
    
    /// Event emitted when new IP is registered
    struct IPRegisteredEvent has copy, drop {
        ip_id: address,
        creator: address,
        title: String,
        ip_type: u8,
        created_at: u64,
    }
    
    /// Event emitted when a license is granted
    struct LicenseGrantedEvent has copy, drop {
        license_id: address,
        ip_id: address,
        licensee: address,
        license_type: u8,
        granted_at: u64,
        expires_at: u64,
        payment_amount: u64,
    }
    
    /// Event emitted when a license status changes
    struct LicenseStatusChangedEvent has copy, drop {
        license_id: address,
        ip_id: address,
        licensee: address,
        old_status: u8,
        new_status: u8,
    }
    
    /// Event emitted when a dispute is created
    struct DisputeCreatedEvent has copy, drop {
        dispute_id: address,
        ip_id: address,
        challenger: address,
        original_creator: address,
        created_at: u64,
    }
    
    /// Event emitted when a dispute is resolved
    struct DisputeResolvedEvent has copy, drop {
        dispute_id: address,
        ip_id: address,
        resolution: u8,
        resolver: address,
    }
    
    /// Initialize the IP Registry - should be called once on module init
    fun init_module(ctx: &mut TxContext) {
        let registry = IPRegistry {
            id: object::new(ctx),
            creator_ips: table::new(ctx),
            ip_by_type: table::new(ctx),
            total_ips: 0,
            admin: tx_context::sender(ctx),
        };
        
        // Initialize tables for all IP types
        table::add(&mut registry.ip_by_type, IP_TYPE_CONTENT, vector::empty<address>());
        table::add(&mut registry.ip_by_type, IP_TYPE_IMAGE, vector::empty<address>());
        table::add(&mut registry.ip_by_type, IP_TYPE_AUDIO, vector::empty<address>());
        table::add(&mut registry.ip_by_type, IP_TYPE_VIDEO, vector::empty<address>());
        table::add(&mut registry.ip_by_type, IP_TYPE_CODE, vector::empty<address>());
        table::add(&mut registry.ip_by_type, IP_TYPE_DESIGN, vector::empty<address>());
        table::add(&mut registry.ip_by_type, IP_TYPE_OTHER, vector::empty<address>());
        
        // Share the registry as a shared object
        transfer::share_object(registry);
    }
    
    /// Create and register a new IP
    public fun create_myip(
        registry: &mut IPRegistry,
        creator_profile: &Profile,
        title: String,
        description: String,
        ip_type: u8,
        content_url: Option<vector<u8>>,
        content_hash: vector<u8>,
        metadata: String,
        poc_id: Option<address>,
        transferable: bool,
        royalty_basis_points: u16,
        registered_countries: vector<String>,
        ctx: &mut TxContext
    ): MyIP {
        // Verify IP type is valid
        assert!(
            ip_type == IP_TYPE_CONTENT || 
            ip_type == IP_TYPE_IMAGE || 
            ip_type == IP_TYPE_AUDIO || 
            ip_type == IP_TYPE_VIDEO || 
            ip_type == IP_TYPE_CODE || 
            ip_type == IP_TYPE_DESIGN || 
            ip_type == IP_TYPE_OTHER, 
            EInvalidIPType
        );
        
        let creator_id = object::uid_to_address(profile::id(creator_profile));
        
        // Convert content URL bytes to Url if provided
        let url = if (option::is_some(&content_url)) {
            let url_bytes = option::extract(&mut content_url);
            option::some(url::new_unsafe_from_bytes(url_bytes))
        } else {
            option::none()
        };
        
        let myip = MyIP {
            id: object::new(ctx),
            creator: creator_id,
            title,
            description,
            ip_type,
            content_url: url,
            content_hash,
            metadata,
            poc_id,
            created_at: tx_context::epoch(ctx),
            transferable,
            royalty_basis_points,
            registered_countries,
            ipo_tokenized: false,
        };
        
        let ip_id = object::uid_to_address(&myip.id);
        
        // Update registry
        if (!table::contains(&registry.creator_ips, creator_id)) {
            table::add(&mut registry.creator_ips, creator_id, vector::empty<address>());
        };
        
        let creator_ips = table::borrow_mut(&mut registry.creator_ips, creator_id);
        vector::push_back(creator_ips, ip_id);
        
        let ips_by_type = table::borrow_mut(&mut registry.ip_by_type, ip_type);
        vector::push_back(ips_by_type, ip_id);
        
        // Increment total IPs counter
        registry.total_ips = registry.total_ips + 1;
        
        // Emit registration event
        event::emit(IPRegisteredEvent {
            ip_id,
            creator: creator_id,
            title: myip.title,
            ip_type: myip.ip_type,
            created_at: myip.created_at,
        });
        
        myip
    }
    
    /// Create and register a new IP asset and transfer to the creator
    public entry fun register_ip(
        registry: &mut IPRegistry,
        creator_profile: &Profile,
        title: String,
        description: String,
        ip_type: u8,
        content_url: vector<u8>,
        content_hash: vector<u8>,
        metadata: String,
        poc_id: Option<address>,
        transferable: bool,
        royalty_basis_points: u16,
        registered_countries: vector<String>,
        ctx: &mut TxContext
    ) {
        let url_opt = if (vector::length(&content_url) > 0) {
            option::some(content_url)
        } else {
            option::none()
        };
        
        let myip = create_myip(
            registry,
            creator_profile,
            title,
            description,
            ip_type,
            url_opt,
            content_hash,
            metadata,
            poc_id,
            transferable,
            royalty_basis_points,
            registered_countries,
            ctx
        );
        
        // Transfer IP to the creator
        transfer::transfer(myip, tx_context::sender(ctx));
    }
    
    /// Grant a license for an IP
    public entry fun grant_license(
        ip: &MyIP,
        licensor_profile: &Profile,
        licensee: address,
        license_type: u8,
        terms: String,
        duration_epochs: u64,
        payment: &mut Coin<MYS>,
        payment_amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify the licensor is the IP creator
        let licensor_id = object::uid_to_address(profile::id(licensor_profile));
        assert!(licensor_id == ip.creator, EUnauthorized);
        
        // Verify payment
        assert!(payment_amount == 0 || coin::value(payment) >= payment_amount, EInvalidPayment);
        
        let now = clock::timestamp_ms(clock) / 1000; // Convert ms to seconds
        let expires_at = if (duration_epochs == 0) { 
            0 // Perpetual license
        } else {
            now + duration_epochs
        };
        
        let license = License {
            id: object::new(ctx),
            ip_id: object::uid_to_address(&ip.id),
            license_type,
            terms,
            licensee,
            granted_at: now,
            expires_at,
            status: LICENSE_ACTIVE,
            payment_amount,
        };
        
        // Process payment if required
        if (payment_amount > 0) {
            let payment_coin = coin::split(payment, payment_amount, ctx);
            // Transfer payment to IP creator
            transfer::public_transfer(payment_coin, ip.creator);
        };
        
        let license_id = object::uid_to_address(&license.id);
        
        // Emit license granted event
        event::emit(LicenseGrantedEvent {
            license_id,
            ip_id: license.ip_id,
            licensee: license.licensee,
            license_type: license.license_type,
            granted_at: license.granted_at,
            expires_at: license.expires_at,
            payment_amount: license.payment_amount,
        });
        
        // Transfer license to licensee
        transfer::transfer(license, licensee);
    }
    
    /// Update license status
    public entry fun update_license_status(
        license: &mut License,
        new_status: u8,
        licensor_profile: &Profile,
        ip: &MyIP,
        ctx: &mut TxContext
    ) {
        // Verify the licensor is the IP creator
        let licensor_id = object::uid_to_address(profile::id(licensor_profile));
        assert!(licensor_id == ip.creator, EUnauthorized);
        assert!(license.ip_id == object::uid_to_address(&ip.id), ELicenseNotFound);
        
        let old_status = license.status;
        license.status = new_status;
        
        // Emit status changed event
        event::emit(LicenseStatusChangedEvent {
            license_id: object::uid_to_address(&license.id),
            ip_id: license.ip_id,
            licensee: license.licensee,
            old_status,
            new_status,
        });
    }
    
    /// File a dispute for an IP
    public entry fun file_dispute(
        ip: &MyIP,
        challenger_profile: &Profile,
        reason: String,
        evidence: String,
        ctx: &mut TxContext
    ) {
        let challenger_id = object::uid_to_address(profile::id(challenger_profile));
        
        // Create dispute
        let dispute = IPDispute {
            id: object::new(ctx),
            ip_id: object::uid_to_address(&ip.id),
            challenger: challenger_id,
            original_creator: ip.creator,
            reason,
            evidence,
            created_at: tx_context::epoch(ctx),
            status: DISPUTE_ACTIVE,
            resolution: 0, // No resolution yet
            resolver: option::none(),
            resolution_notes: string::utf8(b""),
        };
        
        let dispute_id = object::uid_to_address(&dispute.id);
        
        // Emit dispute created event
        event::emit(DisputeCreatedEvent {
            dispute_id,
            ip_id: dispute.ip_id,
            challenger: dispute.challenger,
            original_creator: dispute.original_creator,
            created_at: dispute.created_at,
        });
        
        // Share the dispute as a shared object
        transfer::share_object(dispute);
    }
    
    /// Resolve an IP dispute (admin only)
    public entry fun resolve_dispute(
        registry: &IPRegistry,
        dispute: &mut IPDispute,
        resolution: u8,
        resolution_notes: String,
        ctx: &mut TxContext
    ) {
        // Verify admin
        assert!(registry.admin == tx_context::sender(ctx), EUnauthorized);
        assert!(dispute.status == DISPUTE_ACTIVE, EDisputeNotActive);
        
        // Verify resolution is valid
        assert!(
            resolution == RESOLUTION_ORIGINAL_CREATOR || 
            resolution == RESOLUTION_CHALLENGER || 
            resolution == RESOLUTION_SHARED, 
            EInvalidResolution
        );
        
        dispute.status = DISPUTE_RESOLVED;
        dispute.resolution = resolution;
        dispute.resolver = option::some(tx_context::sender(ctx));
        dispute.resolution_notes = resolution_notes;
        
        // Emit dispute resolved event
        event::emit(DisputeResolvedEvent {
            dispute_id: object::uid_to_address(&dispute.id),
            ip_id: dispute.ip_id,
            resolution,
            resolver: tx_context::sender(ctx),
        });
        
        // Note: In a real implementation, we would need to update the IP ownership
        // if the resolution favors the challenger
    }
    
    // === Getters ===
    
    /// Get IP creator
    public fun creator(ip: &MyIP): address {
        ip.creator
    }
    
    /// Get IP title
    public fun title(ip: &MyIP): String {
        ip.title
    }
    
    /// Get IP description
    public fun description(ip: &MyIP): String {
        ip.description
    }
    
    /// Get IP type
    public fun ip_type(ip: &MyIP): u8 {
        ip.ip_type
    }
    
    /// Get IP content URL
    public fun content_url(ip: &MyIP): &Option<Url> {
        &ip.content_url
    }
    
    /// Get IP content hash
    public fun content_hash(ip: &MyIP): vector<u8> {
        ip.content_hash
    }
    
    /// Get IP metadata
    public fun metadata(ip: &MyIP): String {
        ip.metadata
    }
    
    /// Get IP PoC ID
    public fun poc_id(ip: &MyIP): &Option<address> {
        &ip.poc_id
    }
    
    /// Set the Proof of Creativity ID for an IP
    public fun set_poc_id(ip: &mut MyIP, poc_id: address) {
        ip.poc_id = option::some(poc_id);
    }
    
    /// Get IP created timestamp
    public fun created_at(ip: &MyIP): u64 {
        ip.created_at
    }
    
    /// Check if IP is transferable
    public fun is_transferable(ip: &MyIP): bool {
        ip.transferable
    }
    
    /// Get IP royalty basis points
    public fun royalty_basis_points(ip: &MyIP): u16 {
        ip.royalty_basis_points
    }
    
    /// Get registered countries
    public fun registered_countries(ip: &MyIP): vector<String> {
        ip.registered_countries
    }
    
    /// Check if IP is tokenized
    public fun is_tokenized(ip: &MyIP): bool {
        ip.ipo_tokenized
    }
    
    /// Get license status
    public fun license_status(license: &License): u8 {
        license.status
    }
    
    /// Get license terms
    public fun license_terms(license: &License): String {
        license.terms
    }
    
    /// Check if license is active
    public fun is_license_active(license: &License): bool {
        license.status == LICENSE_ACTIVE
    }
    
    /// Get dispute status
    public fun dispute_status(dispute: &IPDispute): u8 {
        dispute.status
    }
    
    /// Get dispute resolution
    public fun dispute_resolution(dispute: &IPDispute): u8 {
        dispute.resolution
    }
}