MySocial indexer is an off-fullnode service to serve data from MySocial protocol, including both data directly generated from chain and derivative data.

&#9888; **Warning:** MySocial indexer is still experimental and we expect occasional breaking changes that require backfills.

## Architecture
![enhanced_FN](https://user-images.githubusercontent.com/106119108/221022505-a1d873c6-60e2-45f1-b2aa-e50192c4dfbb.png)

## Steps to run locally
### Prerequisites
- install local [Postgres server](https://www.postgresql.org/download/). You can also `brew install postgresql@15` and then add the following to your `~/.zshrc` or `~/.zprofile`, etc:
```sh
export LDFLAGS="-L/opt/homebrew/opt/postgresql@15/lib"
export CPPFLAGS="-I/opt/homebrew/opt/postgresql@15/include"
export PATH="/opt/homebrew/opt/postgresql@15/bin:$PATH"
```
- make sure you have libpq installed: `brew install libpq`, and in your profile, add `export PATH="/opt/homebrew/opt/libpq/bin:$PATH"`. If this doesn't work, try `brew link --force libpq`.

- install Diesel CLI with `cargo install diesel_cli --no-default-features --features postgres`, refer to [Diesel Getting Started guide](https://diesel.rs/guides/getting-started) for more details
- [optional but handy] Postgres client like [Postico](https://eggerapps.at/postico2/), for local check, query execution etc.

### Start the Postgres Service

Postgres must run as a service in the background for other tools to communicate with.  If it was installed using homebrew, it can be started as a service with:

``` sh
brew services start postgresql@version
```

### Local Development(Recommended)

See the [docs](https://docs.mysocial.network/guides/developer/getting-started/local-network) for detailed information. Below is a quick start guide:

Start a local network using the `mys` binary:
```sh
cargo run --bin mys -- start --with-faucet --force-regenesis
```

If you want to run a local network with the indexer enabled (note that `libpq` is required), you can run the following command after following the steps in the next section to set up an indexer DB:
```sh
cargo run --bin mys -- start --with-faucet --force-regenesis --with-indexer --pg-port 5432 --pg-db-name mys_indexer_v2
```

### Running standalone indexer
1. DB setup, under `mys/crates/mys-indexer` run:
```sh
# an example DATABASE_URL is "postgres://postgres:postgres@localhost/exampledb"
diesel setup --database-url="<DATABASE_URL>"
diesel database reset --database-url="<DATABASE_URL>"
```
Note that you need an existing database for this to work. Using the DATABASE_URL example in the comment of the previous code, replace `exampledb` with the name of your database.

2. Checkout to your target branch

For example, if you want to be on the DevNet branch
```sh
git fetch upstream devnet && git reset --hard upstream/devnet
```
3. Start indexer binary, under `mys/crates/mys-indexer` run:
- run indexer as a writer, which pulls data from fullnode and writes data to DB
```sh
# Change the RPC_CLIENT_URL to http://0.0.0.0:9000 to run indexer against local validator & fullnode
cargo run --bin mys-indexer -- --db-url "<DATABASE_URL>" --rpc-client-url "https://fullnode.devnet.mysocial.network:443" --fullnode-sync-worker --reset-db
```
- run indexer as a reader, which is a JSON RPC server with the [interface](https://docs.mysocial.network/mys-api-ref#mysx_getallbalances)
```
cargo run --bin mys-indexer -- --db-url "<DATABASE_URL>" --rpc-client-url "https://fullnode.devnet.mysocial.network:443" --rpc-server-worker
```
More flags info can be found in this [file](src/main.rs#L41).

### DB reset
When making db-related changes, you may find yourself having to run migrations and reset dbs often. The commands below are how you can invoke these actions.
```sh
cargo run --bin mys-indexer -- --database-url "<DATABASE_URL>" reset-database --force
```

## Steps to run locally (TiDB)

### Prerequisites

1. Install TiDB

``` sh
curl --proto '=https' --tlsv1.2 -sSf https://tiup-mirrors.pingcap.com/install.sh | sh
```

2. Install a compatible version of MySQL (At the time of writing, this is MySQL 8.0 -- note that 8.3 is incompatible).

``` sh
brew install mysql@8.0
```

3. Install a version of `diesel_cli` that supports MySQL (and probably also Postgres). This version of the CLI needs to be built against the version of MySQL that was installed in the previous step (compatible with the local installation of TiDB, 8.0.37 at time of writing).

``` sh
MYSQLCLIENT_LIB_DIR=/opt/homebrew/Cellar/mysql@8.0/8.0.37/lib/ cargo install diesel_cli --no-default-features --features postgres --features mysql --force
```

### Run the indexer

1.Run TiDB

```sh
tiup playground
```

2.Verify tidb is running by connecting to it using the mysql client, create database `test`

```sh
mysql --comments --host 127.0.0.1 --port 4000 -u root
create database test;
```

3.DB setup, under `mys/crates/mys-indexer` run:

```sh
# an example DATABASE_URL is "mysql://root:password@127.0.0.1:4000/test"
diesel setup --database-url="<DATABASE_URL>" --migration-dir='migrations/mysql'
diesel database reset --database-url="<DATABASE_URL>" --migration-dir='migrations/mysql'
```

Note that you need an existing database for this to work. Using the DATABASE_URL example in the comment of the previous code, replace `test` with the name of your database.
4. Run indexer as a writer, which pulls data from fullnode and writes data to DB

```sh
# Change the RPC_CLIENT_URL to http://0.0.0.0:9000 to run indexer against local validator & fullnode
cargo run --bin mys-indexer --features mysql-feature --no-default-features -- --db-url "<DATABASE_URL>" --rpc-client-url "https://fullnode.devnet.mysocial.network:443" --fullnode-sync-worker --reset-db
```

### Extending the indexer

To add a new table, run `diesel migration generate your_table_name`, and modify the newly created `up.sql` and `down.sql` files.

You would apply the migration with `diesel migration run`, and run the script in `./scripts/generate_indexer_schema.sh` to update the `schema.rs` file.

## API Endpoints

### Common Query Parameters

Many endpoints support common pagination and filtering parameters:
- `page` (optional) - Page number for pagination (default: 1)
- `limit` (optional) - Results per page, max 100 (default: 20)
- `offset` (optional) - Pagination offset (alternative to page)

### Health Check
- **GET /health** - Check the indexer's health

### Statistics
- **GET /stats/system** - Get overall system statistics (tokens, posts, comments, reactions, relationships)

### Search API
- **GET /search** - Global search across profiles, posts, spt tokens, spt staking pools, governance circles, platforms, mydata, and governance proposals

#### Search Parameters
- `query` (required) - Search term to match against various fields
- `page` (optional) - Page number for pagination (default: 1)
- `limit` (optional) - Results per page, max 100 (default: 20)
- `filter_types` (optional) - Comma-separated list of entity types to include

#### Searchable Entity Types
- `profile` - Search profiles by username, address, and bio
- `post` - Search posts by creativity, post ID, owner, and profile ID
- `spt-token` - Search social proof token pools by name, symbol, pool ID, owner, and associated ID
- `spt-reservaton-pool` - Search staking pools by pool ID, associated ID, owner, and status
- `governance-registry` - Search governance circles/registries (ecosystem, reputation, community notes) with delegate counts and voting parameters
- `platform` - Search platforms by name, platform ID, and developer address
- `mydata` - Search MyData entries by ID, owner, media type, and tags
- `proposal` - Search governance proposals by title, description, ID, and submitter

### Profile API

**Note**: All endpoints with `:id` parameters accept wallet addresses (owner_address) only. Profile IDs are not accepted as input parameters.

- **GET /profiles** - List profiles
- **GET /profiles/address/:address** - Get profile by blockchain address
- **GET /profiles/username/:username** - Get profile by username
- **GET /profiles/username/:username/availability** - Check if a username is available for registration
- **GET /profiles/:id/posts** - Get posts by a profile
  - `:id` parameter accepts wallet address (owner_address)
- **GET /profiles/:id/events** - Get profile events
  - `:id` parameter accepts wallet address (owner_address)
- **GET /profiles/:id/platforms** - Get platform membership events (history)
  - `:id` parameter accepts wallet address (owner_address)
- **GET /profiles/:id/platform-memberships** - Get all platforms a profile is currently a member of
  - Query: `limit` (optional), `offset` (optional), `page` (optional), `search` (optional - searches platform name, platform ID, or tagline)
  - `:id` parameter accepts wallet address (owner_address)
- **GET /profiles/:id/blocking** - Get blocking history
  - `:id` parameter accepts wallet address (owner_address)

### Profile Badge API

**Note**: All endpoints accept wallet addresses (owner_address) only. Profile IDs are not accepted as input parameters.

- **GET /profiles/:id/badges** - Get all badges for a specific profile
  - `:id` parameter accepts wallet address (owner_address)
  - Query: `limit` (optional, default: 20, max: 100), `offset` (optional, default: 0), `platform_id` (optional), `revoked` (optional - filter by revoked status), `badge_type` (optional - filter by badge type/tier)
- **GET /badges/:badge_id** - Get a specific badge by badge_id
  - Query: `profile_id` (required - wallet address that uniquely identifies the badge)
- **GET /badges** - List all badges across all profiles with optional filtering
  - Query: `limit` (optional, default: 20, max: 100), `offset` (optional, default: 0), `profile_id` (optional - wallet address), `platform_id` (optional), `revoked` (optional), `badge_type` (optional)

### Social Graph API

**Note**: All endpoints accept wallet addresses (owner_address) only. Profile IDs are not accepted as input parameters.

- **GET /profiles/:id/following** - List profiles followed by a profile
  - `:id` parameter accepts wallet address (owner_address)
  - Query: `viewer_id` (optional), `limit`, `offset`, `page`
  - Query: `sort` (latest | earliest | alphabetical; default latest), `search` (matches username, display name, or wallet address)
- **GET /profiles/:id/followers** - List followers of a profile
  - `:id` parameter accepts wallet address (owner_address)
  - Query: `viewer_id` (optional), `limit`, `offset`, `page`
  - Query: `sort` (latest | earliest | alphabetical; default latest), `search` (matches username, display name, or wallet address)
- **GET /profiles/:id/stats** - Get follow statistics
  - `:id` parameter accepts wallet address (owner_address)
- **GET /social-graph/check/:follower/:following** - Check if a profile follows another
  - Both `:follower` and `:following` parameters accept wallet addresses (owner_address)
- **GET /social-graph/chart-data** - Get social graph chart data

### Blocking API

**Note**: All endpoints accept wallet addresses (owner_address) only. Profile IDs are not accepted as input parameters.

- **GET /profiles/:id/blocked** - List profiles blocked by a profile
  - `:id` parameter accepts wallet address (owner_address)
  - Query: `sort` (latest | earliest | alphabetical; default latest), `search` (matches username, display name, or wallet address)
- **GET /profiles/:id/blocked-platforms** - List platforms blocked by a profile
  - `:id` parameter accepts wallet address (owner_address)
- **GET /blocklist/check/profile/:blocker/:blocked** - Check if a profile is blocked
  - Both `:blocker` and `:blocked` parameters accept wallet addresses (owner_address)
- **GET /blocklist/check/platform/:profile/:platform** - Check if a platform is blocked
  - `:profile` parameter accepts wallet address (owner_address)

### Platform API
- **GET /platforms** - List platforms
  - Query: `limit` (optional), `offset` (optional), `page` (optional), `primary_category` (optional - filter by primary category), `secondary_category` (optional - filter by secondary category)
- **GET /platforms/approved** - List approved platforms
  - Query: `limit` (optional), `offset` (optional), `page` (optional), `primary_category` (optional - filter by primary category), `secondary_category` (optional - filter by secondary category)
- **GET /platforms/:id** - Get platform by ID
- **GET /platforms/:id/moderators** - Get platform moderators
- **GET /platforms/:id/approval** - Get platform approval status
- **GET /platforms/:id/blocked** - Get profiles blocked by a platform
  - Query: `limit` (optional), `offset` (optional), `page` (optional), `search` (optional - searches usernames and wallet addresses)
- **GET /platforms/:id/members** - Get platform members with profile information
  - Query: `limit` (optional), `offset` (optional), `page` (optional), `search` (optional - searches usernames and wallet addresses)
- **GET /platforms/:id/membership/:profile_id** - Check if a profile is a member of a platform
  - `:profile_id` parameter accepts wallet address (owner_address)
- **GET /platforms/:id/events** - Get platform events with pagination and optional event type filtering
  - Query: `limit` (optional, default: 50), `offset` (optional, default: 0), `page` (optional, default: 1), `event_type` (optional - filter by specific event type)

### Post API
- **GET /posts** - List posts
- **GET /posts/:id** - Get post by ID
- **GET /posts/:id/comments** - Get comments for a post
- **GET /posts/:id/reactions** - Get reactions for a post
- **GET /posts/:id/reposts** - Get reposts of a post
- **GET /posts/trending** - Get trending posts
- **GET /posts/configuration** - Get current post configuration (PostAdminCap settings: prediction settings, content limits, tip percentages, etc.)

### Post Promotion API
- **GET /promotions** - List promoted posts with optional filtering
- **GET /posts/:id/promotion** - Get promotion details for a specific post
- **GET /promotions/:id/views** - Get views for a specific promotion
- **GET /promotions/:id/stats** - Get detailed statistics for a promotion

### TimescaleDB-Optimized Promotion Analytics
- **GET /promotions/:id/analytics/time-series** - Get hourly time-bucketed analytics using TimescaleDB time_bucket
- **GET /promotions/:id/analytics/hourly** - Get hourly stats from continuous aggregates for better performance
- **GET /promotions/analytics/top-performing** - Get top performing promotions from materialized views
- **GET /promotions/analytics/spending-trends** - Get platform-wide spending trends from continuous aggregates

### Proof of Creativity (PoC) API
- **GET /poc/badges** - List all proof of creativity badges
- **GET /poc/badges/:id** - Get specific proof of creativity badge details
- **GET /poc/revenue-redirections** - List all revenue redirections
- **GET /poc/analysis-results** - List AI analysis results for creativity
- **GET /poc/disputes** - List all creativity disputes
- **GET /poc/disputes/:id** - Get specific dispute details
- **GET /poc/disputes/:id/votes** - Get votes for a specific dispute
- **GET /poc/analytics** - Get proof of creativity analytics
- **GET /poc/configuration** - Get current PoC system configuration
- **GET /posts/:id/poc-badges** - Get PoC badges for a specific post
- **GET /posts/:id/revenue-redirections** - Get revenue redirections for a specific post

### Subscription API
- **GET /subscriptions** - List all subscriptions
- **GET /subscription-services** - List all subscription services
- **GET /subscription-revenue** - Get subscription revenue data
- **GET /subscriptions/:id/status** - Get subscription status
- **GET /subscription-access/:subscriber/:content_id** - Check subscription access
- **GET /subscription-analytics** - Get subscription analytics
- **GET /service-performance** - Get service performance metrics
- **GET /subscribers/:address/summary** - Get subscriber summary for an address

### Token Vesting API
- **GET /vesting/wallets** - List all vesting wallets with optional filtering by owner. Returns wallet data with profile information (username, fullname, profile_photo)
- **GET /vesting/wallets/active** - Get all active vesting wallets ordered by highest token holding (active = has started, hasn't ended, has remaining balance > 0). Returns wallet data with profile information
- **GET /vesting/wallets/:wallet_id** - Get specific vesting wallet details with real-time status
- **GET /vesting/wallets/:wallet_id/events** - Get complete event history for a vesting wallet
- **GET /vesting/wallets/:wallet_id/claimable** - Get real-time claimable amount with progress details
- **GET /vesting/users/:address/wallets** - Get all vesting wallets for a specific user address. Returns wallet data with profile information
- **GET /vesting/events** - List all vesting events with optional owner filtering
- **GET /vesting/analytics** - Get platform-wide vesting statistics and metrics
- **GET /vesting/leaderboard** - Get vesting leaderboard (top users by vested amounts)

#### Vesting Query Parameters
- `limit` (optional) - Number of results per page (default: 50)
- `offset` (optional) - Pagination offset (default: 0)
- `page` (optional) - Page number for pagination (default: 1)
- `owner_address` (optional) - Filter by wallet owner address

### Revenue Analytics API
- **GET /revenue/dashboard** - Get comprehensive revenue dashboard
- **GET /revenue/leaderboard** - Get revenue leaderboard
- **GET /revenue/chart-data** - Get revenue chart data for visualization
- **GET /revenue/unified** - Get unified revenue across all sources
- **GET /revenue/creators/:address/stats** - Get creator revenue statistics
- **GET /revenue/platforms/:address/stats** - Get platform revenue statistics
- **GET /revenue/spt/pools/:pool_id** - Get SPT pool revenue

### MyData Marketplace API
- **GET /mydata** - List MyData entries
- **GET /mydata/configuration** - Get current MyData marketplace configuration
- **GET /mydata/popular** - Get popular MyData entries
- **GET /mydata/:id** - Get MyData entry by ID
- **GET /mydata/:id/purchases** - Get purchases for a MyData entry
- **GET /mydata/:id/subscriptions** - Get subscriptions for a MyData entry
- **GET /mydata/:id/revenue** - Get revenue for a MyData entry
- **GET /mydata/:id/access-logs** - Get access logs for a MyData entry
- **GET /mydata/:id/stats** - Get statistics for a MyData entry
- **GET /mydata/:id/revenue-timeline** - Get revenue timeline for a MyData entry
- **GET /mydata/:id/access-analytics** - Get access analytics for a MyData entry
- **GET /creators/:id/mydata** - Get MyData entries created by a specific address

### Governance API
- **GET /governance/proposals** - List governance proposals
- **GET /governance/proposals/:id** - Get proposal details
- **GET /governance/proposals/:id/votes** - Get community votes on a proposal
- **GET /governance/proposals/:id/anonymous-stats** - Get anonymous voting statistics for a proposal
- **GET /governance/proposals/:id/anonymous-votes** - Get anonymous votes for a proposal
- **GET /governance/proposals/:id/decryption-failures** - Get vote decryption failures for a proposal
- **GET /governance/delegates** - List delegates
- **GET /governance/delegates/:address** - Get delegate details
- **GET /governance/delegates/:address/proposals** - Get proposals reviewed by a delegate
- **GET /governance/delegates/:address/ratings** - Get ratings for a delegate
- **GET /governance/nominees** - List nominated delegates
- **GET /governance/registries** - List governance registries
- **GET /governance/registries/:registry_type** - Get registry by type
- **GET /governance/events** - List recent governance events
- **GET /governance/anonymous-voting-trends** - Get anonymous voting trends analytics

### Social Proof Token API

#### Token Pool Management
- **GET /social-proof-token/pools** - List token pools
  - Query: `page` (optional, default: 1), `limit` (optional, default: 20, max: 100), `token_type` (optional - filter by token type), `owner` (optional - filter by owner address), `sort_by` (optional - "created", "supply", or "price"), `sort_dir` (optional - "asc" or "desc")
- **GET /social-proof-token/pools/:id** - Get token pool by ID
- **GET /social-proof-token/pools/by-associated-id/:id** - Get token pool by associated profile or post ID
- **GET /social-proof-token/pools/:id/transactions** - Get transactions for a token pool
  - Query: `page` (optional, default: 1), `limit` (optional, default: 20, max: 100)
- **GET /social-proof-token/pools/:id/holdings** - Get holdings for a token pool
  - Query: `page` (optional, default: 1), `limit` (optional, default: 20, max: 100)
- **GET /social-proof-token/pools/:id/price-history** - Get price history for a token pool
  - Query: `from` (optional - Unix timestamp in seconds), `to` (optional - Unix timestamp in seconds), `interval` (optional - "hour", "day", "week", or "month", default: "hour")
- **GET /social-proof-token/pools/:id/liquidity-profile** - Show transaction volume, frequency and depth to assess token liquidity

#### Staking System
- **GET /social-proof-token/reservation-pools** - List active reservation pools supporting posts/profiles
  - Query: `page` (optional, default: 1), `limit` (optional, default: 20, max: 100)
  - Returns pools with status 'active' or 'threshold_met', ordered by total_reserved DESC
- **GET /social-proof-token/reservation-pools/:id** - Get reservation pool details by pool ID
  - `:id` parameter accepts pool_id OR associated_id (profile_0x... or post_0x...)
  - Returns enhanced reservation pool data with fee breakdowns (total_fees_paid, total_creator_fees, total_platform_fees, total_treasury_fees), reservation_count, and unique_reservers
- **GET /social-proof-token/reservation-pools/:id/reservations** - Get individual reservations for a pool
  - `:id` parameter accepts pool_id OR associated_id (profile_0x... or post_0x...)
  - Query: `page` (optional, default: 1), `limit` (optional, default: 20, max: 100)
  - Returns latest reservation per reserver address, ordered by amount DESC

#### Configuration
- **GET /social-proof-token/configuration** - Get current social proof tokens exchange configuration (thresholds, fees, trading status, etc.)
  - Returns configuration including ecosystem_treasury address

#### Analytics & Insights
- **GET /social-proof-token/popular** - Get popular token pools
  - Query: `page` (optional, default: 1), `limit` (optional, default: 20, max: 100)
- **GET /social-proof-token/users/:address/holdings** - Get token holdings for a user
  - Query: `include_reservations` (optional, boolean - if true, includes reservation data with fee breakdowns)
  - Returns enhanced holdings with total_value and optional reservations array
- **GET /social-proof-token/analytics/top-performers** - Get tokens with highest price/volume growth in specified period
  - Query: `period` (optional - "day", "week", or "month", default: "day")
- **GET /social-proof-token/portfolios/:address/performance** - Track user's token portfolio value over time with ROI metrics
  - Query: `from` (optional - Unix timestamp in seconds), `to` (optional - Unix timestamp in seconds)
  - Returns portfolio performance with holdings, value_history, and ROI calculations
- **GET /social-proof-token/creators/:address/revenue-streams** - Break down creator revenue from token fees across content
  - Query: `from` (optional - Unix timestamp in seconds), `to` (optional - Unix timestamp in seconds)
  - Returns revenue breakdown by token pool and time period
- **GET /social-proof-token/market-sentiment** - Aggregate buy/sell patterns to create market momentum indicators
  - Returns overall sentiment, volume metrics, and sentiment by token type

### Social Proof of Truth (SPoT) API
- **GET /spot/configuration** - Get current Social Proof of Truth configuration
- **GET /spot/:post_id/record** - Get SPoT state for a post (status, outcome, escrow totals)
- **GET /spot/:post_id/bets** - List SPoT bets for a post
- **GET /spot/:post_id/payouts** - List SPoT payouts made to winning participants
- **GET /spot/:post_id/refunds** - List SPoT refunds issued on unresolved or draw outcomes

#### Query Parameters
- `page` (optional) - Page number for pagination (default: 1)
- `limit` (optional) - Results per page, max 100 (default: 20)

### Insurance API
- **GET /insurance/config** - Get current insurance system configuration
- **GET /insurance/vaults** - List all insurance vaults
- **GET /insurance/vaults/:vault_id** - Get specific vault details
- **GET /insurance/vaults/:vault_id/transactions** - Get transaction history for a vault
- **GET /insurance/vaults/:vault_id/exposures** - Get exposure details for a vault
- **GET /insurance/policies** - List all insurance policies
- **GET /insurance/policies/:policy_id** - Get specific policy details
- **GET /insurance/markets/:market_id/policies** - Get policies for a specific market

### Treasury API
- **GET /treasury/current** - Get current treasury state
- **GET /treasury/history** - Get treasury history
