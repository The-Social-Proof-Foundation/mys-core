// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use bip32::XPub;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel_async::AsyncConnection;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::RunQueryDsl;

use crate::address_book::derive_evm_address_from_xpub;
use crate::address_index::AddressIndex;
use crate::models::{NewEvmDerivationCounter, NewEvmDepositAddress};
use crate::postgres_manager::PgPool;
use crate::schema::{evm_deposit_addresses, evm_derivation_counters};

/// Get (or allocate) an EVM deposit address for a Mys user.
///
/// - Mapping is immutable: (chain_name, evm_address) and (chain_name, mys_address) are unique.
/// - Allocation is serialized per chain via `evm_derivation_counters` row lock.
pub async fn get_or_create_deposit_address(
    pool: &PgPool,
    chain_name: &str,
    mys_address: &[u8],
    xpub: &XPub,
) -> Result<[u8; 20]> {
    let mut conn = pool.get().await?;

    let chain_name_owned = chain_name.to_string();
    let mys_address_vec = mys_address.to_vec();

    let evm_addr = conn
        .transaction(|conn| {
            async move {
                // 1) Return existing mapping if present.
                let existing: Option<Vec<u8>> = evm_deposit_addresses::table
                    .filter(evm_deposit_addresses::chain_name.eq(&chain_name_owned))
                    .filter(evm_deposit_addresses::mys_address.eq(&mys_address_vec))
                    .select(evm_deposit_addresses::evm_address)
                    .first::<Vec<u8>>(conn)
                    .await
                    .optional()?;

                if let Some(bytes) = existing {
                    let fixed: [u8; 20] = bytes
                        .as_slice()
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("Stored evm_address is not 20 bytes"))?;
                    return Ok::<[u8; 20], anyhow::Error>(fixed);
                }

                // 2) Ensure counter row exists.
                diesel::insert_into(evm_derivation_counters::table)
                    .values(NewEvmDerivationCounter {
                        chain_name: &chain_name_owned,
                        next_index: 0,
                    })
                    .on_conflict(evm_derivation_counters::chain_name)
                    .do_nothing()
                    .execute(conn)
                    .await?;

                // 3) Lock counter row and allocate a derivation index.
                let next_index: i64 = evm_derivation_counters::table
                    .filter(evm_derivation_counters::chain_name.eq(&chain_name_owned))
                    .for_update()
                    .select(evm_derivation_counters::next_index)
                    .first(conn)
                    .await?;

                diesel::update(
                    evm_derivation_counters::table
                        .filter(evm_derivation_counters::chain_name.eq(&chain_name_owned)),
                )
                .set(evm_derivation_counters::next_index.eq(next_index + 1))
                .execute(conn)
                .await?;

                // 4) Derive EVM address from xpub and insert mapping.
                let eth_addr = derive_evm_address_from_xpub(xpub, next_index as u32)?;
                let eth_bytes: [u8; 20] = eth_addr.0;

                diesel::insert_into(evm_deposit_addresses::table)
                    .values(NewEvmDepositAddress {
                        chain_name: &chain_name_owned,
                        mys_address: &mys_address_vec,
                        derivation_index: next_index,
                        evm_address: eth_bytes.as_slice(),
                    })
                    .execute(conn)
                    .await?;

                Ok::<[u8; 20], anyhow::Error>(eth_bytes)
            }
            .scope_boxed()
        })
        .await?;

    Ok(evm_addr)
}

/// Load all known deposit addresses for a chain into an in-memory index.
pub async fn load_address_index(pool: &PgPool, chain_name: &str) -> Result<AddressIndex> {
    let mut conn = pool.get().await?;
    let rows: Vec<(i64, Vec<u8>, Vec<u8>)> = evm_deposit_addresses::table
        .filter(evm_deposit_addresses::chain_name.eq(chain_name))
        .select((
            evm_deposit_addresses::id,
            evm_deposit_addresses::evm_address,
            evm_deposit_addresses::mys_address,
        ))
        .order_by(evm_deposit_addresses::id.asc())
        .load(&mut conn)
        .await?;

    let mut idx = AddressIndex::new(chain_name.to_string());
    for (id, evm, mys) in rows {
        let evm20: [u8; 20] = evm
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Stored evm_address is not 20 bytes"))?;
        let mys32: [u8; 32] = mys
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Stored mys_address is not 32 bytes"))?;
        idx.apply_row(id, evm20, mys32);
    }
    Ok(idx)
}

/// Incrementally refresh an address index by loading rows with id > last_loaded_id.
pub async fn refresh_address_index(pool: &PgPool, idx: &mut AddressIndex) -> Result<()> {
    let mut conn = pool.get().await?;
    let rows: Vec<(i64, Vec<u8>, Vec<u8>)> = evm_deposit_addresses::table
        .filter(evm_deposit_addresses::chain_name.eq(&idx.chain_name))
        .filter(evm_deposit_addresses::id.gt(idx.last_loaded_id))
        .select((
            evm_deposit_addresses::id,
            evm_deposit_addresses::evm_address,
            evm_deposit_addresses::mys_address,
        ))
        .order_by(evm_deposit_addresses::id.asc())
        .load(&mut conn)
        .await?;

    for (id, evm, mys) in rows {
        let evm20: [u8; 20] = evm
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Stored evm_address is not 20 bytes"))?;
        let mys32: [u8; 32] = mys
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Stored mys_address is not 32 bytes"))?;
        idx.apply_row(id, evm20, mys32);
    }
    Ok(())
}
