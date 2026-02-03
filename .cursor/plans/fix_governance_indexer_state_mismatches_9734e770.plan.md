---
name: Fix Governance Indexer State Mismatches
overview: "Fix multiple state mismatches between Move contract and indexer: registry type constants, delegate-rejected refunds, vote count idempotency, and voting timestamp handling."
todos:
  - id: fix-registry-constants
    content: Fix registry type constants to match Move (add PROOF_OF_CREATIVITY=1, PLATFORM=3, remove incorrect REPUTATION/COMMUNITY_NOTES)
    status: completed
  - id: fix-search-labels
    content: Fix hardcoded registry type labels in search handler
    status: completed
  - id: zero-reward-pool-rejection
    content: Zero reward_pool when processing ProposalRejectedEvent (delegate rejection)
    status: completed
  - id: document-quorum-limitation
    content: Document quorum-not-met refund limitation in code comments
    status: completed
  - id: document-epoch-timestamps
    content: Add comments to voting time fields indicating they are epochs, not timestamps
    status: pending
  - id: fix-vote-count-idempotency
    content: Fix vote count increment to only happen on insert, not update (idempotency)
    status: pending
isProject: false
---

# Fix Governance Indexer State Mismatches

## Issues Analysis

After reviewing the code, here are the findings:

### 1. Registry Type Constants Mismatch (REAL ISSUE - Needs Fix)

**Move Contract** (`governance.move:53-55`):

- `PROPOSAL_TYPE_ECOSYSTEM = 0`
- `PROPOSAL_TYPE_PROOF_OF_CREATIVITY = 1`
- `PROPOSAL_TYPE_PLATFORM = 3`

**Indexer** (`mod.rs:36-38`):

- `GOVERNANCE_REGISTRY_ECOSYSTEM = 0` ✓
- `GOVERNANCE_REGISTRY_REPUTATION = 1` ✗ (doesn't exist in Move)
- `GOVERNANCE_REGISTRY_COMMUNITY_NOTES = 2` ✗ (doesn't exist in Move)
- Missing `GOVERNANCE_REGISTRY_PLATFORM = 3` ✗

**Impact**: 

- Constants are defined but not actively used for comparisons (good - less breaking)
- `search.rs:242-245` has hardcoded labels that are wrong: "Reputation Registry" for type 1, "Community Notes Registry" for type 2
- Platform handler correctly hardcodes `3` for platform type
- If constants are used in future code, they'll be wrong

**Fix**: Update constants to match Move and fix search handler labels.

### 2. Delegate-Rejected Proposal Refund (REAL ISSUE - Needs Fix)

**Move Contract** (`governance.move:1364-1374`):

- `reject_proposal_by_id` refunds `reward_pool` to submitter
- Emits `ProposalRejectedEvent` but event doesn't include refund info

**Indexer** (`governance_events.rs:776-901`):

- `process_proposal_rejected_event` updates status but **doesn't zero reward_pool**
- No refund record created

**Impact**: Database shows funds still locked when they're actually refunded on-chain.

**Fix**: Zero `reward_pool` when processing `ProposalRejectedEvent`.

### 3. Quorum-Not-Met Refund (DESIGN LIMITATION - Needs Decision)

**Move Contract** (`governance.move:1616-1641`):

- When quorum isn't met, refunds `reward_pool` to submitter
- Emits same `ProposalRejectedByCommunityEvent` as normal rejection
- Event doesn't indicate whether refund happened

**Indexer**: Cannot distinguish quorum-not-met from normal rejection.

**Recommended Solution**: Extend `ProposalRejectedByCommunityEvent` in Move contract to include:

- `quorum_met: bool` - indicates if quorum was reached, OR
- `refund_amount: u64` - amount refunded (0 if rewards distributed to voters)

This allows indexer to update `reward_pool` correctly without chain queries.

**Fallback** (if Move change not possible now):

- Document limitation in code comments
- Indexer will show incorrect `reward_pool` for quorum-not-met rejections

### 4. Voting Timestamps Are Epochs (MINOR ISSUE - Document or Convert)

**Move Contract** (`governance.move:1307-1308, 1331-1332`):

- Uses epoch numbers: `voting_start_time = current_epoch`
- Comment: "using epochs" not milliseconds

**Indexer** (`governance_events.rs:729-732`):

- Stores as `i64` without conversion
- Field names suggest timestamps (`voting_start_time`, `voting_end_time`)

**Impact**: API consumers expecting millisecond timestamps will get epoch numbers instead.

**Options**:

1. Document in API/model comments that these are epochs
2. Convert epochs to timestamps (requires epoch duration lookup)
3. Rename fields to `voting_start_epoch` / `voting_end_epoch`

**Recommendation**: Document in model comments. Conversion requires additional complexity and epoch duration lookup.

### 5. Vote Count Idempotency (REAL ISSUE - Needs Fix)

**Problem** (`governance_events.rs:513-548, 629-667`):

- `on_conflict do_update` handles duplicate votes
- But vote counts are **unconditionally incremented** after insert/update
- If event is replayed, counts get inflated

**Example**:

```rust
diesel::insert_into(...)
    .on_conflict(...)
    .do_update()  // Updates existing vote
    .execute(...)?;

// This always increments, even if vote already existed!
proposals::delegate_approval_count.eq(delegate_approval_count + 1)
```

**Impact**: Event replay causes vote count inflation, desyncing from on-chain totals.

**Fix**: Only increment if vote was actually inserted (not updated). Use `get_result()` to check if insert or update occurred, or check existence before incrementing.

## Implementation Plan

### Step 1: Fix Registry Type Constants

**File**: `crates/mys-indexer/src/social/mod.rs`

**Change**:

```rust
// Remove incorrect constants:
// pub const GOVERNANCE_REGISTRY_REPUTATION: u8 = 1;
// pub const GOVERNANCE_REGISTRY_COMMUNITY_NOTES: u8 = 2;

// Add correct constant:
pub const GOVERNANCE_REGISTRY_PROOF_OF_CREATIVITY: u8 = 1;
pub const GOVERNANCE_REGISTRY_PLATFORM: u8 = 3;
```

**File**: `crates/mys-indexer/src/social/api/handlers/search.rs`

**Change** (lines 242-245):

```rust
// Fix hardcoded labels to match Move contract
WHEN registry_type = 0 THEN 'Ecosystem Registry'
WHEN registry_type = 1 THEN 'Proof of Creativity Registry'
WHEN registry_type = 3 THEN 'Platform Registry'
ELSE 'Governance Registry'
```

### Step 2: Zero Reward Pool on Delegate Rejection

**File**: `crates/mys-indexer/src/social/events/governance_events.rs`

**Change** in `process_proposal_rejected_event` (after line 796):

```rust
// Update proposal status and zero reward pool (refunded on-chain)
diesel::update(crate::social::schema::proposals::table)
    .filter(crate::social::schema::proposals::id.eq(&proposal_id))
    .set((
        crate::social::schema::proposals::status.eq(GOVERNANCE_STATUS_REJECTED as i16),
        crate::social::schema::proposals::reward_pool.eq(0), // Refunded on-chain
    ))
    .execute(tx_conn)
    .await?;
```

### Step 3: Handle Quorum-Not-Met Refund

**Option A: If Move Contract Extended** (Recommended)

**Move Contract** (`governance.move`): Extend `ProposalRejectedByCommunityEvent` struct:

```move
public struct ProposalRejectedByCommunityEvent has copy, drop {
    proposal_id: ID,
    rejection_time: u64,
    votes_for: u64,
    votes_against: u64,
    quorum_met: bool,  // NEW: indicates if quorum was reached
}
```

**File**: `crates/mys-indexer/src/social/events/governance_event_types.rs`

**Add field** to `ProposalRejectedByCommunityEvent`:

```rust
pub struct ProposalRejectedByCommunityEvent {
    // ... existing fields ...
    pub quorum_met: bool,  // or refund_amount: u64
}
```

**File**: `crates/mys-indexer/src/social/events/governance_events.rs`

**Update** `process_proposal_rejected_by_community_event` to zero reward_pool when quorum not met:

```rust
// Zero reward_pool if quorum not met (refunded to submitter)
// Otherwise, reward_pool was distributed to voters
if !rejected_event.quorum_met {
    diesel::update(crate::social::schema::proposals::table)
        .filter(crate::social::schema::proposals::id.eq(&proposal_id))
        .set(crate::social::schema::proposals::reward_pool.eq(0))
        .execute(tx_conn)
        .await?;
}
```

**Option B: If Move Contract Not Changed** (Fallback)

**File**: `crates/mys-indexer/src/social/events/governance_events.rs`

**Add comment** documenting limitation:

```rust
/// Process a proposal rejected by community event
/// 
/// LIMITATION: This event is emitted both when:
/// - Quorum is met but proposal is rejected (rewards distributed to voters)
/// - Quorum is not met (reward_pool refunded to submitter)
/// 
/// The event doesn't indicate which case occurred. The indexer cannot distinguish
/// without querying chain state. 
/// 
/// BEST FIX: Extend ProposalRejectedByCommunityEvent in Move contract to include
/// quorum_met: bool or refund_amount: u64.
```

### Step 4: Document Voting Timestamps Are Epochs

**File**: `crates/mys-indexer/src/social/models/governance.rs`

**Add comments** to voting time fields:

```rust
/// Voting start time (epoch number, not milliseconds)
pub voting_start_time: Option<i64>,
/// Voting end time (epoch number, not milliseconds)  
pub voting_end_time: Option<i64>,
```

### Step 5: Fix Vote Count Idempotency

**File**: `crates/mys-indexer/src/social/events/governance_events.rs`

**Change** in `process_delegate_vote_event`:

```rust
// Check if vote already exists before incrementing
let vote_exists = crate::social::schema::delegate_votes::table
    .filter(
        crate::social::schema::delegate_votes::proposal_id.eq(&vote_event.proposal_id)
            .and(crate::social::schema::delegate_votes::delegate_address.eq(&vote_event.delegate_address))
    )
    .count()
    .get_result::<i64>(tx_conn)
    .await? > 0;

// Insert or update vote
diesel::insert_into(...)
    .on_conflict(...)
    .do_update()
    .execute(tx_conn)
    .await?;

// Only increment if this was a new vote
if !vote_exists {
    // Update proposal vote counts
    if vote_event.approve {
        diesel::update(...)
            .set(delegate_approval_count.eq(delegate_approval_count + 1))
            .execute(tx_conn)
            .await?;
    } else {
        diesel::update(...)
            .set(delegate_rejection_count.eq(delegate_rejection_count + 1))
            .execute(tx_conn)
            .await?;
    }
}
```

**Apply same pattern** to `process_community_vote_event` (but note: community votes use `vote_weight`, so need to track previous weight to adjust correctly).

**Alternative for community votes**: Since `vote_weight` can change, we need to:

1. Get previous vote_weight if exists
2. Subtract old weight, add new weight
3. Or use a SQL update that calculates the difference

## Questions for User

1. **Quorum-not-met refund**: Can you extend the Move contract's `ProposalRejectedByCommunityEvent` to include `quorum_met: bool` or `refund_amount: u64`? This is the best fix. If not possible now, we'll document the limitation as fallback.
2. **Voting timestamps**: Prefer documenting as epochs, or converting to timestamps (requires epoch duration lookup)?
3. **Community vote idempotency**: Community votes can change `vote_weight` on update. Should we:
  - Track previous weight and adjust difference?
  - Or only increment on first insert (simpler but loses weight updates)?

