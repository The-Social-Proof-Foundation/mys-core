// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0
// @generated manually (keep in sync with migrations)

// Diesel doesn't support `NUMERIC` without bigdecimal by default, but the schema
// can still reference it via `Numeric`.

diesel::table! {
     evm_deposit_addresses (id) {
         id -> Int8,
         chain_name -> Text,
         mys_address -> Bytea,
         derivation_index -> Int8,
         evm_address -> Bytea,
         created_at -> Timestamptz,
     }
 }

diesel::table! {
     evm_deposits (id) {
         id -> Int8,

         chain_name -> Text,
         asset_id -> Bytea,
         token_kind -> Text,
         token_address -> Nullable<Bytea>,

         tx_hash -> Bytea,
         log_index -> Int4,
         block_number -> Int8,

         from_address -> Nullable<Bytea>,
         to_address -> Bytea,
         mys_address -> Bytea,

         amount_wei -> Numeric,

         deposit_hash -> Bytea,

         status -> Text,
         observed_at -> Timestamptz,
         finalized_at -> Nullable<Timestamptz>,
         credited_at -> Nullable<Timestamptz>,
         myso_tx_digest -> Nullable<Bytea>,
     }
 }

diesel::table! {
     evm_scanner_progress (id) {
         id -> Int8,
         chain_name -> Text,
         scanner_name -> Text,
         last_scanned_block -> Int8,
         last_finalized_block -> Int8,
         updated_at -> Timestamptz,
     }
 }

diesel::table! {
    evm_derivation_counters (chain_name) {
        chain_name -> Text,
        next_index -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
     evm_deposit_addresses,
     evm_deposits,
     evm_scanner_progress,
    evm_derivation_counters,
 );
