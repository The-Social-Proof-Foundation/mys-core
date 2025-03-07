# mys-indexer-alt

## Running
The required flags are --remote-store-url (or --local-ingestion-path) and the --config. If both are provided, remote-store-url will be used.

```
cargo run --bin mys-indexer-alt -- --database-url {url} indexer --remote-store-url https://checkpoints.mainnet.mys.io --skip-watermark --first-checkpoint 68918060 --last-checkpoint 68919060 --config indexer_alt_config.toml
```

## Pruning
To enable pruning, the `cp_sequence_numbers` pipeline must be enabled. Otherwise, even if pruning logic is
configured for a table, the pruner task itself will skip if it cannot find a mapping for the
checkpoint pruning watermark. Only one committer needs to update this table - it is not necessary
for every indexer instance to have this pipeline enabled.
