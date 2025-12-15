CREATE TABLE IF NOT EXISTS evm_deposits (
  id BIGSERIAL PRIMARY KEY,

  chain_name TEXT NOT NULL,
  asset_id BYTEA NOT NULL,           -- 32 bytes (canonical asset key)
  token_kind TEXT NOT NULL,          -- 'native' | 'erc20'
  token_address BYTEA,              -- 20 bytes for ERC20, NULL for native

  tx_hash BYTEA NOT NULL,            -- 32 bytes
  log_index INT NOT NULL DEFAULT -1, -- ERC20 log index, -1 for native
  block_number BIGINT NOT NULL,

  from_address BYTEA,
  to_address BYTEA NOT NULL,         -- 20 bytes
  mys_address BYTEA NOT NULL,        -- 32 bytes

  amount_wei NUMERIC NOT NULL,       -- store U256 safely

  deposit_hash BYTEA NOT NULL,       -- 32 bytes, on-chain idempotency key

  status TEXT NOT NULL,              -- 'observed' | 'finalized' | 'credited' | 'failed'
  observed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  finalized_at TIMESTAMPTZ,
  credited_at TIMESTAMPTZ,
  myso_tx_digest BYTEA,

  CONSTRAINT evm_deposits_dedupe UNIQUE (chain_name, tx_hash, log_index)
);

CREATE INDEX IF NOT EXISTS evm_deposits_chain_status_idx ON evm_deposits (chain_name, status);
CREATE INDEX IF NOT EXISTS evm_deposits_to_address_idx ON evm_deposits (to_address);
CREATE INDEX IF NOT EXISTS evm_deposits_mys_address_idx ON evm_deposits (mys_address);
CREATE INDEX IF NOT EXISTS evm_deposits_asset_id_idx ON evm_deposits (asset_id);
CREATE INDEX IF NOT EXISTS evm_deposits_block_number_idx ON evm_deposits (chain_name, block_number);
CREATE INDEX IF NOT EXISTS evm_deposits_deposit_hash_idx ON evm_deposits (deposit_hash);
