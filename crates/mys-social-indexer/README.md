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
- **Social Proof Token**: Tracks token pools, transactions, holdings, and auctions
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
CHECKPOINT_URL=https://checkpoints.testnet.mysocial.network
START_CHECKPOINT=0
INDEXER_CONCURRENCY=5

# Package configuration
PROFILE_PACKAGE_ADDRESS=0xe5759970ebb63cb02e34af3304a61600b07ed3cbd10376b3a0be98952b54aa76

# Logging
RUST_LOG=info,mys_social_indexer=debug
```

## API Endpoints

### Health Check
- **GET /health** - Check the indexer's health

### Search API
- **GET /search** - Global search across profiles, posts, tokens, platforms, myip, and governance proposals

### Profile API
- **GET /profiles** - List profiles
- **GET /profiles/address/:address** - Get profile by blockchain address
- **GET /profiles/username/:username** - Get profile by username
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

### MyIP API (Intellectual Property)
- **GET /licenses** - List intellectual property licenses
- **GET /licenses/popular** - Get popular licenses
- **GET /licenses/:id** - Get license by ID
- **GET /licenses/:id/events** - Get events for a license
- **GET /licenses/:id/grants** - Get grants for a license
- **GET /licenses/:id/revenue** - Get revenue for a license
- **GET /licenses/:id/posts** - Get posts using a license
- **GET /licenses/:id/stats** - Get statistics for a license
- **GET /licenses/:id/revenue-timeline** - Get revenue timeline for a license
- **GET /creators/:id/licenses** - Get licenses created by an address

### Governance API
- **GET /governance/proposals** - List governance proposals
- **GET /governance/proposals/:id** - Get proposal details
- **GET /governance/proposals/:id/votes** - Get community votes on a proposal
- **GET /governance/delegates** - List delegates
- **GET /governance/delegates/:address** - Get delegate details
- **GET /governance/delegates/:address/proposals** - Get proposals reviewed by a delegate
- **GET /governance/delegates/:address/ratings** - Get ratings for a delegate
- **GET /governance/nominees** - List nominated delegates 
- **GET /governance/registries** - List governance registries
- **GET /governance/registries/:registry_type** - Get registry by type
- **GET /governance/events** - List recent governance events

### Social Proof Token API
- **GET /social-proof-token/pools** - List token pools
- **GET /social-proof-token/pools/:id** - Get token pool by ID
- **GET /social-proof-token/pools/by-associated-id/:id** - Get token pool by associated profile or post ID
- **GET /social-proof-token/pools/:id/transactions** - Get transactions for a token pool
- **GET /social-proof-token/pools/:id/holdings** - Get holdings for a token pool
- **GET /social-proof-token/pools/:id/price-history** - Get price history for a token pool
- **GET /social-proof-token/auctions** - List active token auctions
- **GET /social-proof-token/auctions/:id** - Get auction details by ID
- **GET /social-proof-token/auctions/:id/contributions** - Get contributions for an auction
- **GET /social-proof-token/popular** - Get popular token pools
- **GET /social-proof-token/users/:address/holdings** - Get token holdings for a user
- **GET /social-proof-token/analytics/top-performers** - Get tokens with highest price/volume growth in specified period
- **GET /social-proof-token/portfolios/:address/performance** - Track user's token portfolio value over time with ROI metrics
- **GET /social-proof-token/creators/:address/revenue-streams** - Break down creator revenue from token fees across content
- **GET /social-proof-token/market-sentiment** - Aggregate buy/sell patterns to create market momentum indicators
- **GET /social-proof-token/pools/:id/liquidity-profile** - Show transaction volume, frequency and depth to assess token liquidity

## License

Apache License 2.0