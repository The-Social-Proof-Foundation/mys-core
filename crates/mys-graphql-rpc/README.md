# mys-graphql-rpc

## Dev setup
Note that we use compilation flags to determine the backend for Diesel. If you're using VS Code, make sure to update settings.json with the appropriate features - there should at least be a "pg_backend" (or other backend.)
```
"rust-analyzer.cargo.features": ["pg_backend"]
```
Consequently, you'll also need to specify the backend when running cargo commands:
```cargo run --features "pg_backend" --bin mys-graphql-rpc start-server --db-url <DB_URL>```

The order is important:
1. --features "pg_backend": This part tells Cargo to enable the pg_backend feature.
2. --bin mys-graphql-rpc: This specifies which binary to run.
3. start-server --db-url: These are arguments to the binary.

## Spinning up locally

### Setting up local db

The graphql service is backed by a db based on the db schema in [mys-indexer](../mys-indexer/src/schema.rs). To spin up a local db, follow the instructions at [mys-indexer](../mys-indexer/README.md) until "Running standalone indexer".

If you have not created a db yet, you can do so as follows:
```sh
psql -U postgres
CREATE DATABASE mys_indexer_v2;
```

You should be able to refer to the db url now:
`psql postgres://postgres:postgrespw@localhost:5432/mys_indexer_v2`

With the new db, run the following commands (also under `mys/crates/mys-indexer`):

```sh
diesel setup --database-url="<DATABASE_URL>" --migration-dir=migrations
diesel migration run --database-url="<DATABASE_URL>" --migration-dir=migrations
```

### Launching the server
See [src/commands.rs](src/commands.rs) for all CLI options.

Example `.toml` config:
```toml
[limits]
max-query-depth = 15
max-query-nodes = 500
max-output-nodes = 100000
max-query-payload-size = 5000
max-db-query-cost = 20000
default-page-size = 5
max-page-size = 10
request-timeout-ms = 15000
max-type-argument-depth = 16
max-type-argument-width = 32
max-type-nodes = 256
max-move-value-depth = 128

[background-tasks]
watermark-update-ms=500
```

This will build mys-graphql-rpc and start an IDE:
```
cargo run --bin mys-graphql-rpc start-server [--rpc-url] [--db-url] [--port] [--host] [--config]
```

### Launching the server w/ indexer
For local dev, it might be useful to spin up an indexer as well. Instructions are at [Running standalone indexer](../mys-indexer/README.md#running-standalone-indexer).

## Compatibility with json-rpc

`cargo run --bin mys -- start --with-faucet --force-regenesis --with-indexer --pg-port 5432 --pg-db-name mys_indexer_v2 --with-graphql`

`pnpm --filter @mysten/graphql-transport test:e2e`

## Testing
The full gamut of graphql-specific tests are listed in the [rust.yml](../../.github/workflows/rust.yml).

To run the tests in `mys-graphql-rpc`, you will need to have postgres running locally.
