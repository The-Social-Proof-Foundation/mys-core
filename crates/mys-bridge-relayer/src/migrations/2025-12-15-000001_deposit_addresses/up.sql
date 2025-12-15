CREATE TABLE IF NOT EXISTS evm_deposit_addresses (
  id BIGSERIAL PRIMARY KEY,
  chain_name TEXT NOT NULL,
  mys_address BYTEA NOT NULL,
  derivation_index BIGINT NOT NULL,
  evm_address BYTEA NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT evm_deposit_addresses_chain_mys_unique UNIQUE (chain_name, mys_address),
  CONSTRAINT evm_deposit_addresses_chain_evm_unique UNIQUE (chain_name, evm_address),
  CONSTRAINT evm_deposit_addresses_chain_derivation_unique UNIQUE (chain_name, derivation_index)
);

-- Common query patterns
CREATE INDEX IF NOT EXISTS evm_deposit_addresses_chain_name_idx ON evm_deposit_addresses (chain_name);
CREATE INDEX IF NOT EXISTS evm_deposit_addresses_mys_address_idx ON evm_deposit_addresses (mys_address);
CREATE INDEX IF NOT EXISTS evm_deposit_addresses_evm_address_idx ON evm_deposit_addresses (evm_address);
