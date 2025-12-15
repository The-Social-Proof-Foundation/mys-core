CREATE TABLE IF NOT EXISTS evm_scanner_progress (
  id BIGSERIAL PRIMARY KEY,
  chain_name TEXT NOT NULL,
  scanner_name TEXT NOT NULL,
  last_scanned_block BIGINT NOT NULL,
  last_finalized_block BIGINT NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT evm_scanner_progress_unique UNIQUE (chain_name, scanner_name)
);

CREATE INDEX IF NOT EXISTS evm_scanner_progress_chain_idx ON evm_scanner_progress (chain_name);
