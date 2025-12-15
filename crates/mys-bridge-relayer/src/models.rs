// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use bigdecimal::BigDecimal;
use diesel::data_types::PgTimestamp;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::schema::{evm_deposit_addresses, evm_deposits, evm_derivation_counters, evm_scanner_progress};

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = evm_deposit_addresses)]
pub struct EvmDepositAddress {
    pub id: i64,
    pub chain_name: String,
    pub mys_address: Vec<u8>,
    pub derivation_index: i64,
    pub evm_address: Vec<u8>,
    pub created_at: PgTimestamp,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = evm_deposit_addresses)]
pub struct NewEvmDepositAddress<'a> {
    pub chain_name: &'a str,
    pub mys_address: &'a [u8],
    pub derivation_index: i64,
    pub evm_address: &'a [u8],
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = evm_deposits)]
pub struct EvmDeposit {
    pub id: i64,

    pub chain_name: String,
    pub asset_id: Vec<u8>,
    pub token_kind: String,
    pub token_address: Option<Vec<u8>>,

    pub tx_hash: Vec<u8>,
    pub log_index: i32,
    pub block_number: i64,

    pub from_address: Option<Vec<u8>>,
    pub to_address: Vec<u8>,
    pub mys_address: Vec<u8>,

    pub amount_wei: BigDecimal,

    pub deposit_hash: Vec<u8>,

    pub status: String,
    pub observed_at: PgTimestamp,
    pub finalized_at: Option<PgTimestamp>,
    pub credited_at: Option<PgTimestamp>,
    pub myso_tx_digest: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = evm_scanner_progress)]
pub struct EvmScannerProgress {
    pub id: i64,
    pub chain_name: String,
    pub scanner_name: String,
    pub last_scanned_block: i64,
    pub last_finalized_block: i64,
    pub updated_at: PgTimestamp,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = evm_derivation_counters, primary_key(chain_name))]
pub struct EvmDerivationCounter {
    pub chain_name: String,
    pub next_index: i64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = evm_derivation_counters)]
pub struct NewEvmDerivationCounter<'a> {
    pub chain_name: &'a str,
    pub next_index: i64,
}
