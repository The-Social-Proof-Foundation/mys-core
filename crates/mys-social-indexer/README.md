# MySocial Social Indexer

A indexer for the MySocial blockchain that focuses on tracking social interactions.

## Features

- **Profile Indexing**: Tracks profile creation and updates
- **Social Graph Indexing**: Tracks follow/unfollow relationships
- **Platform Indexing**: Tracks platform creation and user membership
- **Post Indexing**: Tracks posts, comments, reactions, tips, and reposts
- **Post Promotion**: Tracks pay-per-view promoted posts with budget management and view tracking
- **MyIP Integration**: Tracks intellectual property licenses and revenue
- **Governance Integration**: Tracks proposals, voting, and delegates
- **Social Proof Token**: Tracks token pools, transactions, holdings, and staking pools
- **Database Storage**: Stores all data in TimescaleDB (PostgreSQL)
- **REST API**: Provides endpoints for accessing indexed data
- **Configurable**: Customizable via environment variables
- **Containerized**: Easy deployment with Docker

## Architecture

The indexer consists of the following components:

1. **Blockchain Listener**: Processes MySocial blockchain checkpoints and extracts events
2. **Event Processor**: Identifies and processes events from various modules
3. **Database**: Stores indexed data in TimescaleDB (PostgreSQL)
4. **API Server**: Exposes indexed data via REST API endpoints

## Social Proof Token Staking System

The indexer now supports a modern staking-based social proof token system, replacing the previous auction model. Users can stake MYS tokens towards posts and profiles to enable social proof token creation when thresholds are met.

### How It Works
1. **Stake Creation**: Users stake MYS tokens towards posts (1,000 MYS threshold) or profiles (10,000 MYS threshold)
2. **Threshold Monitoring**: The indexer tracks total staked amounts and monitors threshold achievement
3. **Token Creation**: When thresholds are met, post/profile owners can create social proof tokens
4. **Real-time Tracking**: All staking activity is tracked in real-time with comprehensive analytics

### Key Features
- **Stake Pool Management**: Track total staked amounts per post/profile with real-time status updates
- **Individual Stake Tracking**: Monitor user stakes with history of deposits and withdrawals
- **Threshold Achievement**: Automatic detection when posts/profiles meet staking requirements
- **Analytics**: Comprehensive staking metrics including trends, velocity, and top pools

### Database Tables (TimescaleDB Hypertables)
- `spt_stake_pools`: Stake pool configurations and current totals (1-month chunks)
- `spt_stakes`: Individual stake records with full history (1-week chunks)
- `spt_exchange_config`: Exchange configuration changes and threshold updates (1-month chunks)

### Staking Thresholds
- **Posts**: 1,000 MYS tokens required to enable social proof token creation
- **Profiles**: 10,000 MYS tokens required to enable social proof token creation
- **Individual Limits**: Maximum 20% of threshold per individual staker

### API Capabilities
- **Real-time Status**: Live staking pool status and threshold progress
- **Stake Analytics**: Comprehensive metrics on staking trends and patterns
- **User Tracking**: Complete stake history per user across all pools
- **Threshold Monitoring**: Track pools approaching or exceeding thresholds

## Post Promotion Feature

The indexer now supports tracking promoted posts, a pay-per-view system where post creators can allocate MYS tokens to promote their content. This feature is fully optimized for TimescaleDB to handle high-volume time-series data efficiently.

### How It Works
1. **Promotion Creation**: When a post is promoted, a `PromotedPostCreatedEvent` is emitted containing the promotion budget and payment per view
2. **View Tracking**: Each confirmed view triggers a `PromotedPostViewConfirmedEvent` that records the viewer, payment amount, and platform
3. **Budget Management**: The indexer tracks remaining budget and automatically updates promotion status
4. **Analytics**: Continuous aggregates provide real-time statistics on promotion performance

### TimescaleDB Features Used
- **Hypertables**: All promotion tables are TimescaleDB hypertables with automatic partitioning by time
- **Continuous Aggregates**: Pre-computed hourly and daily aggregates for instant analytics
- **Compression**: Automatic compression policies reduce storage by up to 90% for older data
- **Time Bucketing**: Efficient time-series queries using `time_bucket` function
- **Materialized Views**: Pre-computed performance metrics for top promotions

### Database Tables (All TimescaleDB Hypertables)
- `promoted_posts`: Stores promotion configurations and current status (1-month chunks)
- `promotion_views`: Time-series data for all confirmed views (1-week chunks)
- `promotion_status_events`: Tracks status changes (1-month chunks)
- `promotion_budget_events`: Records budget additions and spending (1-month chunks)

### Continuous Aggregates
- `promotion_views_hourly`: Hourly rollup of views per promotion (refreshes every hour)
- `promotion_spending_daily`: Daily platform-wide spending metrics (refreshes every 6 hours)

### API Endpoints
- **Basic Queries**: Standard promotion data and statistics
- **Time-Series Analytics**: Leverage TimescaleDB's time_bucket for efficient aggregation
- **Continuous Aggregates**: Query pre-computed data for instant results
- **Performance Views**: Access materialized views for top promotions and trends

## Getting Started

### Prerequisites

- Rust 1.75+
- PostgreSQL 15+
- Docker and Docker Compose (for containerized deployment)

### Running Locally

1. Clone the repository
2. Install dependencies:
   ```
   cargo build
   ```
3. Set up the database:
   ```
   createdb mys_social_indexer
   ```
4. Run the indexer:
   ```
   cargo run
   ```

### Using Docker

```bash
docker-compose up -d
```

This will start:
- PostgreSQL database
- Social Profile Indexer

## Configuration

The indexer can be configured via environment variables:

```bash
# Database configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/myso_social_indexer
DATABASE_MAX_CONNECTIONS=10

# Server configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Indexer configuration
CHECKPOINT_URL=https://mysocial-testnet-checkpoints.storage.googleapis.com
START_CHECKPOINT=0
INDEXER_CONCURRENCY=5

# Package configuration
PROFILE_PACKAGE_ADDRESS=0x000000000000000000000000000000000000000000000000000000000000d880

# Logging
RUST_LOG=info,mys_social_indexer=debug
```

## API Endpoints

### Health Check
- **GET /health** - Check the indexer's health

### Search API
- **GET /search** - Global search across profiles, posts, spt tokens, spt staking pools, governance circles, platforms, myip, and governance proposals

#### Search Parameters
- `query` (required) - Search term to match against various fields
- `page` (optional) - Page number for pagination (default: 1)
- `limit` (optional) - Results per page, max 100 (default: 20)
- `filter_types` (optional) - Comma-separated list of entity types to include

#### Searchable Entity Types
- `profile` - Search profiles by username, address, and bio
- `post` - Search posts by content, post ID, owner, and profile ID
- `spt-token` - Search social proof token pools by name, symbol, pool ID, owner, and associated ID
- `spt-stake-pool` - Search staking pools by pool ID, associated ID, owner, and status
- `governance-registry` - Search governance circles/registries (ecosystem, reputation, community notes) with delegate counts and voting parameters
- `platform` - Search platforms by name, platform ID, developer address, and description
- `proposal` - Search governance proposals by title, description, ID, and submitter

#### Search Features
- **Smart Ranking**: Exact matches appear first, followed by partial matches
- **Rich Metadata**: Each result includes entity-specific metadata (e.g., staking progress, token prices, governance delegate counts)
- **Real-time Status**: Staking pools show current progress toward thresholds
- **Comprehensive Coverage**: Searches across all major system entities

#### Example Queries
```bash
# Search everything
GET /search?query=alice

# Search only staking pools
GET /search?query=threshold_met&filter_types=spt-stake-pool

# Search tokens and staking pools
GET /search?query=0x123&filter_types=spt-token,spt-stake-pool

# Search governance circles
GET /search?query=ecosystem&filter_types=governance-registry

# Search with pagination
GET /search?query=social&page=2&limit=50
```

### Profile API
- **GET /profiles** - List profiles
- **GET /profiles/address/:address** - Get profile by blockchain address
- **GET /profiles/username/:username** - Get profile by username
- **GET /profiles/username/:username/availability** - Check if a username is available for registration
- **GET /profiles/:id/posts** - Get posts by a profile
- **GET /profiles/:id/events** - Get profile events
- **GET /profiles/:id/platforms** - Get platform memberships
- **GET /profiles/:id/blocking** - Get blocking history

### Social Graph API
- **GET /profiles/:id/following** - List profiles followed by a profile
- **GET /profiles/:id/followers** - List followers of a profile
- **GET /profiles/:id/stats** - Get follow statistics
- **GET /social-graph/check/:follower/:following** - Check if a profile follows another

### Blocking API
- **GET /profiles/:id/blocked** - List profiles blocked by a profile
- **GET /profiles/:id/blocked-platforms** - List platforms blocked by a profile
- **GET /blocklist/check/profile/:blocker/:blocked** - Check if a profile is blocked
- **GET /blocklist/check/platform/:profile/:platform** - Check if a platform is blocked

### Platform API
- **GET /platforms** - List platforms
- **GET /platforms/approved** - List approved platforms
- **GET /platforms/:id** - Get platform by ID
- **GET /platforms/:id/moderators** - Get platform moderators
- **GET /platforms/:id/approval** - Get platform approval status
- **GET /platforms/:id/blocked** - Get profiles blocked by a platform

### Post API
- **GET /posts** - List posts
- **GET /posts/:id** - Get post by ID
- **GET /posts/:id/comments** - Get comments for a post
- **GET /posts/:id/reactions** - Get reactions for a post
- **GET /posts/:id/reposts** - Get reposts of a post
- **GET /posts/trending** - Get trending posts

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

### Proof of Content (PoC) API
- **GET /poc/badges** - List all proof of content badges
- **GET /poc/badges/:id** - Get specific proof of content badge details
- **GET /poc/revenue-redirections** - List all revenue redirections
- **GET /poc/analysis-results** - List AI analysis results for content
- **GET /poc/disputes** - List all content disputes
- **GET /poc/disputes/:id** - Get specific dispute details
- **GET /poc/disputes/:id/votes** - Get votes for a specific dispute
- **GET /poc/analytics** - Get proof of content analytics
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

### Revenue Analytics API
- **GET /revenue/dashboard** - Get comprehensive revenue dashboard
- **GET /revenue/leaderboard** - Get revenue leaderboard
- **GET /revenue/chart-data** - Get revenue chart data for visualization
- **GET /revenue/unified** - Get unified revenue across all sources
- **GET /revenue/creators/:address/stats** - Get creator revenue statistics
- **GET /revenue/platforms/:address/stats** - Get platform revenue statistics
- **GET /revenue/spt/pools/:pool_id** - Get SPT pool revenue

### MyIP Marketplace API
- **GET /marketplace** - List marketplace data
- **GET /marketplace/popular** - Get popular marketplace items
- **GET /marketplace/:id** - Get marketplace item by ID
- **GET /marketplace/:id/purchases** - Get purchases for a marketplace item
- **GET /marketplace/:id/subscriptions** - Get subscriptions for a marketplace item
- **GET /marketplace/:id/revenue** - Get revenue for a marketplace item
- **GET /marketplace/:id/access-logs** - Get access logs for a marketplace item
- **GET /marketplace/:id/stats** - Get statistics for a marketplace item
- **GET /marketplace/:id/revenue-timeline** - Get revenue timeline for a marketplace item
- **GET /marketplace/:id/access-analytics** - Get access analytics for a marketplace item
- **GET /creators/:id/marketplace-data** - Get marketplace data created by a specific address

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
- **GET /social-proof-token/pools/:id** - Get token pool by ID
- **GET /social-proof-token/pools/by-associated-id/:id** - Get token pool by associated profile or post ID
- **GET /social-proof-token/pools/:id/transactions** - Get transactions for a token pool
- **GET /social-proof-token/pools/:id/holdings** - Get holdings for a token pool
- **GET /social-proof-token/pools/:id/price-history** - Get price history for a token pool
- **GET /social-proof-token/pools/:id/liquidity-profile** - Show transaction volume, frequency and depth to assess token liquidity

#### Staking System
- **GET /social-proof-token/stake-pools** - List active stake pools supporting posts/profiles
- **GET /social-proof-token/stake-pools/:id** - Get stake pool details by pool ID
- **GET /social-proof-token/stake-pools/:id/stakes** - Get individual stakes for a pool

#### Analytics & Insights
- **GET /social-proof-token/popular** - Get popular token pools
- **GET /social-proof-token/users/:address/holdings** - Get token holdings for a user
- **GET /social-proof-token/analytics/top-performers** - Get tokens with highest price/volume growth in specified period
- **GET /social-proof-token/portfolios/:address/performance** - Track user's token portfolio value over time with ROI metrics
- **GET /social-proof-token/creators/:address/revenue-streams** - Break down creator revenue from token fees across content
- **GET /social-proof-token/market-sentiment** - Aggregate buy/sell patterns to create market momentum indicators

## License

Apache License 2.0