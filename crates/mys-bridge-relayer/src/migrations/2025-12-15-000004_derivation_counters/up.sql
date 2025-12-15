CREATE TABLE IF NOT EXISTS evm_derivation_counters (
  chain_name TEXT PRIMARY KEY,
  next_index BIGINT NOT NULL
);
