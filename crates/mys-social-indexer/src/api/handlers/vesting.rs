// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::db::DbPool;
use crate::models::{VestingEvent, VestingWallet, VestingWalletWithStatus};
use crate::schema::{profiles, vesting_events, vesting_wallets};

// ===========================================================================
// REQUEST/RESPONSE TYPES
// ===========================================================================

/// Query parameters for vesting endpoints
#[derive(Debug, Deserialize)]
pub struct VestingQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    #[serde(default = "default_page")]
    pub page: i64,
    pub owner_address: Option<String>,
}

fn default_limit() -> i64 {
    50
}

fn default_page() -> i64 {
    1
}

/// Response type for vesting wallets list
#[derive(Debug, Serialize)]
pub struct VestingWalletsResponse {
    pub wallets: Vec<VestingWalletWithStatus>,
    pub total: i64,
    pub pagination: PaginationInfo,
}

/// Response type for vesting events list
#[derive(Debug, Serialize)]
pub struct VestingEventsResponse {
    pub events: Vec<VestingEvent>,
    pub total: i64,
    pub pagination: PaginationInfo,
}

/// Pagination information
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub page: i64,
    pub total_pages: i64,
}

/// Response for claimable amount query
#[derive(Debug, Serialize)]
pub struct ClaimableResponse {
    pub wallet_id: String,
    pub claimable_amount: i64,
    pub current_progress: f64,
    pub vesting_status: String,
    pub timestamp: u64,
}

/// Vesting analytics response
#[derive(Debug, Serialize)]
pub struct VestingAnalyticsResponse {
    pub total_wallets: i64,
    pub total_vested_amount: i64,
    pub total_claimed_amount: i64,
    pub total_remaining_amount: i64,
    pub active_wallets: i64,
    pub completed_wallets: i64,
    pub average_vesting_duration: f64,
    pub most_common_curve_factor: i64,
}

/// Vesting leaderboard entry
#[derive(Debug, Serialize)]
pub struct VestingLeaderboardEntry {
    pub owner_address: String,
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub profile_photo: Option<String>,
    pub total_vested: i64,
    pub total_claimed: i64,
    pub active_wallets: i64,
    pub completed_wallets: i64,
}

/// Vesting leaderboard response
#[derive(Debug, Serialize)]
pub struct VestingLeaderboardResponse {
    pub entries: Vec<VestingLeaderboardEntry>,
    pub total: i64,
}

// ===========================================================================
// HANDLER FUNCTIONS
// ===========================================================================

/// Get all vesting wallets with optional filtering
pub async fn get_vesting_wallets(
    Query(query): Query<VestingQuery>,
    State(pool): State<DbPool>,
) -> Result<Json<VestingWalletsResponse>, StatusCode> {
    debug!("Getting vesting wallets with query: {:?}", query);

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Calculate offset from page if provided
    let offset = if query.page > 1 {
        (query.page - 1) * query.limit
    } else {
        query.offset
    };

    // Build the base query
    let mut query_builder = vesting_wallets::table.into_boxed();

    // Apply owner address filter if provided
    if let Some(owner) = &query.owner_address {
        query_builder = query_builder.filter(vesting_wallets::owner_address.eq(owner));
    }

    // Get total count - rebuild the query since BoxedSelectStatement doesn't implement Clone
    let mut count_query = vesting_wallets::table.into_boxed();
    if let Some(owner) = &query.owner_address {
        count_query = count_query.filter(vesting_wallets::owner_address.eq(owner));
    }
    let total = count_query
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting wallets count: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get the actual wallets
    let wallets = query_builder
        .order_by(vesting_wallets::created_at.desc())
        .limit(query.limit)
        .offset(offset)
        .load::<VestingWallet>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting wallets: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert to wallets with status (using current timestamp)
    let current_time = chrono::Utc::now().timestamp_millis() as u64;
    let wallets_with_status: Vec<VestingWalletWithStatus> = wallets
        .into_iter()
        .map(|wallet| VestingWalletWithStatus::from_wallet(wallet, current_time))
        .collect();

    let total_pages = (total as f64 / query.limit as f64).ceil() as i64;

    Ok(Json(VestingWalletsResponse {
        wallets: wallets_with_status,
        total,
        pagination: PaginationInfo {
            total,
            limit: query.limit,
            offset,
            page: query.page,
            total_pages,
        },
    }))
}

/// Get a specific vesting wallet by ID
pub async fn get_vesting_wallet_by_id(
    Path(wallet_id): Path<String>,
    State(pool): State<DbPool>,
) -> Result<Json<VestingWalletWithStatus>, StatusCode> {
    debug!("Getting vesting wallet: {}", wallet_id);

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let wallet = vesting_wallets::table
        .filter(vesting_wallets::wallet_id.eq(&wallet_id))
        .first::<VestingWallet>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting wallet {}: {}", wallet_id, e);
            match e {
                diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    // Convert to wallet with status
    let current_time = chrono::Utc::now().timestamp_millis() as u64;
    let wallet_with_status = VestingWalletWithStatus::from_wallet(wallet, current_time);

    Ok(Json(wallet_with_status))
}

/// Get events for a specific vesting wallet
pub async fn get_vesting_wallet_events(
    Path(wallet_id): Path<String>,
    Query(query): Query<VestingQuery>,
    State(pool): State<DbPool>,
) -> Result<Json<VestingEventsResponse>, StatusCode> {
    debug!("Getting vesting wallet events: {}", wallet_id);

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Calculate offset from page if provided
    let offset = if query.page > 1 {
        (query.page - 1) * query.limit
    } else {
        query.offset
    };

    // Get total count
    let total = vesting_events::table
        .filter(vesting_events::wallet_id.eq(&wallet_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting events count: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get the events
    let events = vesting_events::table
        .filter(vesting_events::wallet_id.eq(&wallet_id))
        .order_by(vesting_events::event_time.desc())
        .limit(query.limit)
        .offset(offset)
        .load::<VestingEvent>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting events: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let total_pages = (total as f64 / query.limit as f64).ceil() as i64;

    Ok(Json(VestingEventsResponse {
        events,
        total,
        pagination: PaginationInfo {
            total,
            limit: query.limit,
            offset,
            page: query.page,
            total_pages,
        },
    }))
}

/// Get claimable amount for a vesting wallet
pub async fn get_vesting_wallet_claimable(
    Path(wallet_id): Path<String>,
    State(pool): State<DbPool>,
) -> Result<Json<ClaimableResponse>, StatusCode> {
    debug!("Getting claimable amount for wallet: {}", wallet_id);

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let wallet = vesting_wallets::table
        .filter(vesting_wallets::wallet_id.eq(&wallet_id))
        .first::<VestingWallet>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting wallet {}: {}", wallet_id, e);
            match e {
                diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    let current_time = chrono::Utc::now().timestamp_millis() as u64;
    let progress = wallet.vesting_progress(current_time);

    // Calculate claimable amount based on the curve factor and current time
    let claimable_amount = calculate_claimable_amount(&wallet, current_time);

    let vesting_status = if !wallet.has_started(current_time) {
        "not_started".to_string()
    } else if wallet.has_ended(current_time) {
        "completed".to_string()
    } else {
        "in_progress".to_string()
    };

    Ok(Json(ClaimableResponse {
        wallet_id: wallet.wallet_id,
        claimable_amount,
        current_progress: progress,
        vesting_status,
        timestamp: current_time,
    }))
}

/// Get all vesting wallets for a specific user
pub async fn get_user_vesting_wallets(
    Path(address): Path<String>,
    Query(query): Query<VestingQuery>,
    State(pool): State<DbPool>,
) -> Result<Json<VestingWalletsResponse>, StatusCode> {
    debug!("Getting vesting wallets for user: {}", address);

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Calculate offset from page if provided
    let offset = if query.page > 1 {
        (query.page - 1) * query.limit
    } else {
        query.offset
    };

    // Get total count
    let total = vesting_wallets::table
        .filter(vesting_wallets::owner_address.eq(&address))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get user vesting wallets count: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get the wallets
    let wallets = vesting_wallets::table
        .filter(vesting_wallets::owner_address.eq(&address))
        .order_by(vesting_wallets::created_at.desc())
        .limit(query.limit)
        .offset(offset)
        .load::<VestingWallet>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get user vesting wallets: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert to wallets with status
    let current_time = chrono::Utc::now().timestamp_millis() as u64;
    let wallets_with_status: Vec<VestingWalletWithStatus> = wallets
        .into_iter()
        .map(|wallet| VestingWalletWithStatus::from_wallet(wallet, current_time))
        .collect();

    let total_pages = (total as f64 / query.limit as f64).ceil() as i64;

    Ok(Json(VestingWalletsResponse {
        wallets: wallets_with_status,
        total,
        pagination: PaginationInfo {
            total,
            limit: query.limit,
            offset,
            page: query.page,
            total_pages,
        },
    }))
}

/// Get all vesting events with optional filtering
pub async fn get_vesting_events(
    Query(query): Query<VestingQuery>,
    State(pool): State<DbPool>,
) -> Result<Json<VestingEventsResponse>, StatusCode> {
    debug!("Getting vesting events with query: {:?}", query);

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Calculate offset from page if provided
    let offset = if query.page > 1 {
        (query.page - 1) * query.limit
    } else {
        query.offset
    };

    // Build the base query
    let mut query_builder = vesting_events::table.into_boxed();

    // Apply owner address filter if provided
    if let Some(owner) = &query.owner_address {
        query_builder = query_builder.filter(vesting_events::owner_address.eq(owner));
    }

    // Get total count - rebuild the query since BoxedSelectStatement doesn't implement Clone
    let mut count_query = vesting_events::table.into_boxed();
    if let Some(owner) = &query.owner_address {
        count_query = count_query.filter(vesting_events::owner_address.eq(owner));
    }
    let total = count_query
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting events count: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get the events
    let events = query_builder
        .order_by(vesting_events::event_time.desc())
        .limit(query.limit)
        .offset(offset)
        .load::<VestingEvent>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting events: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let total_pages = (total as f64 / query.limit as f64).ceil() as i64;

    Ok(Json(VestingEventsResponse {
        events,
        total,
        pagination: PaginationInfo {
            total,
            limit: query.limit,
            offset,
            page: query.page,
            total_pages,
        },
    }))
}

/// Get vesting analytics
pub async fn get_vesting_analytics(
    State(pool): State<DbPool>,
) -> Result<Json<VestingAnalyticsResponse>, StatusCode> {
    debug!("Getting vesting analytics");

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get basic statistics
    let total_wallets = vesting_wallets::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .unwrap_or(0);

    // For now, we'll calculate simple statistics without complex aggregations
    // In production, you'd want to use raw SQL or more sophisticated queries

    // Load all wallets to calculate statistics (not efficient for large datasets)
    let all_wallets = vesting_wallets::table
        .load::<VestingWallet>(&mut conn)
        .await
        .unwrap_or_default();

    let total_vested_amount: i64 = all_wallets.iter().map(|w| w.total_amount).sum();
    let total_claimed_amount: i64 = all_wallets.iter().map(|w| w.claimed_amount).sum();
    let total_remaining_amount: i64 = all_wallets.iter().map(|w| w.remaining_balance).sum();

    let current_time = chrono::Utc::now().timestamp_millis();

    // Count active wallets (started but not finished)
    let active_wallets = all_wallets
        .iter()
        .filter(|w| {
            let start_time = w.start_time;
            let end_time = start_time + w.duration;
            current_time >= start_time && current_time < end_time && w.remaining_balance > 0
        })
        .count() as i64;

    // Count completed wallets
    let completed_wallets = all_wallets
        .iter()
        .filter(|w| w.remaining_balance == 0)
        .count() as i64;

    // Calculate average vesting duration (in days)
    let average_vesting_duration = if !all_wallets.is_empty() {
        let total_duration: i64 = all_wallets.iter().map(|w| w.duration).sum();
        let avg_ms = total_duration as f64 / all_wallets.len() as f64;
        avg_ms / (1000.0 * 60.0 * 60.0 * 24.0) // Convert milliseconds to days
    } else {
        0.0
    };

    // Get most common curve factor
    let mut curve_factors: std::collections::HashMap<i64, usize> = std::collections::HashMap::new();
    for wallet in &all_wallets {
        *curve_factors.entry(wallet.curve_factor).or_insert(0) += 1;
    }
    let most_common_curve_factor = curve_factors
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(factor, _)| factor)
        .unwrap_or(1000);

    Ok(Json(VestingAnalyticsResponse {
        total_wallets,
        total_vested_amount,
        total_claimed_amount,
        total_remaining_amount,
        active_wallets,
        completed_wallets,
        average_vesting_duration,
        most_common_curve_factor,
    }))
}

/// Get vesting leaderboard
pub async fn get_vesting_leaderboard(
    Query(query): Query<VestingQuery>,
    State(pool): State<DbPool>,
) -> Result<Json<VestingLeaderboardResponse>, StatusCode> {
    debug!("Getting vesting leaderboard");

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Calculate offset from page if provided
    let offset = if query.page > 1 {
        (query.page - 1) * query.limit
    } else {
        query.offset
    };

    // Get total count of distinct owners
    let total = vesting_wallets::table
        .select(vesting_wallets::owner_address)
        .distinct()
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to get vesting leaderboard count: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get current time for calculating active wallets
    let current_time = chrono::Utc::now().timestamp_millis() as i64;

    // Load all wallets to aggregate by owner
    let all_wallets = vesting_wallets::table
        .load::<VestingWallet>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to load vesting wallets: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Aggregate by owner_address
    let mut aggregated: HashMap<String, (i64, i64, i64, i64)> = HashMap::new();

    for wallet in &all_wallets {
        let entry = aggregated.entry(wallet.owner_address.clone()).or_insert((0, 0, 0, 0));
        entry.0 += wallet.total_amount; // total_vested
        entry.1 += wallet.claimed_amount; // total_claimed
        
        // Check if wallet is active (started but not finished and has remaining balance)
        let start_time = wallet.start_time;
        let end_time = start_time + wallet.duration;
        if current_time >= start_time && current_time < end_time && wallet.remaining_balance > 0 {
            entry.2 += 1; // active_wallets
        }
        
        // Check if wallet is completed (fully claimed)
        if wallet.remaining_balance == 0 {
            entry.3 += 1; // completed_wallets
        }
    }

    // Convert to Vec and sort by total_vested descending
    let mut leaderboard_data: Vec<(String, i64, i64, i64, i64)> = aggregated
        .into_iter()
        .map(|(addr, (vested, claimed, active, completed))| {
            (addr, vested, claimed, active, completed)
        })
        .collect();
    
    leaderboard_data.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by total_vested descending

    // Apply pagination
    let paginated_data: Vec<(String, i64, i64, i64, i64)> = leaderboard_data
        .into_iter()
        .skip(offset as usize)
        .take(query.limit as usize)
        .collect();

    // Get owner addresses for profile lookup
    let owner_addresses: Vec<String> = paginated_data.iter().map(|(addr, _, _, _, _)| addr.clone()).collect();

    // Load profiles for these addresses
    let profiles_map: HashMap<String, (Option<String>, Option<String>, Option<String>)> = if !owner_addresses.is_empty() {
        profiles::table
            .filter(profiles::owner_address.eq_any(&owner_addresses))
            .select((
                profiles::owner_address,
                profiles::username,
                profiles::display_name,
                profiles::profile_photo,
            ))
            .load::<(String, String, Option<String>, Option<String>)>(&mut conn)
            .await
            .map_err(|e| {
                error!("Failed to load profiles: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .into_iter()
            .map(|(addr, username, display_name, profile_photo)| {
                (addr, (Some(username), display_name, profile_photo))
            })
            .collect()
    } else {
        HashMap::new()
    };

    // Build entries with profile data
    let entries: Vec<VestingLeaderboardEntry> = paginated_data
        .into_iter()
        .map(|(owner_address, total_vested, total_claimed, active_wallets, completed_wallets)| {
            let (username, fullname, profile_photo) = profiles_map
                .get(&owner_address)
                .cloned()
                .unwrap_or((None, None, None));
            
            VestingLeaderboardEntry {
                owner_address,
                username,
                fullname,
                profile_photo,
                total_vested,
                total_claimed,
                active_wallets,
                completed_wallets,
            }
        })
        .collect();

    Ok(Json(VestingLeaderboardResponse { entries, total }))
}

// ===========================================================================
// UTILITY FUNCTIONS
// ===========================================================================

/// Calculate claimable amount based on vesting schedule and curve factor
/// This matches the calculation in the smart contract (profile.move)
fn calculate_claimable_amount(wallet: &VestingWallet, current_time_ms: u64) -> i64 {
    let current_time = current_time_ms as i64;

    // If vesting hasn't started yet, nothing is claimable
    if current_time < wallet.start_time {
        return 0;
    }

    // If vesting period is complete, all remaining balance is claimable
    if current_time >= wallet.start_time + wallet.duration {
        return wallet.remaining_balance;
    }

    // Calculate progress through vesting period (0.0 to 1.0)
    let elapsed = current_time - wallet.start_time;
    let progress = elapsed as f64 / wallet.duration as f64;

    // Normalize curve factor (1000 = linear)
    let curve_factor_normalized = wallet.curve_factor as f64 / 1000.0;
    
    // Apply curve based on curve factor (matching smart contract logic)
    let curved_progress = if wallet.curve_factor == 0 || wallet.curve_factor == 1000 {
        // Linear vesting
        progress
    } else if wallet.curve_factor > 1000 {
        // Exponential curve (more tokens toward end)
        // Use quadratic approximation: progress^2
        let quadratic = progress * progress;
        // Blend with linear based on how far curve_factor is from 1000
        let steepness = curve_factor_normalized - 1.0;
        let blend_factor = (steepness * 2.0).min(1.0);
        progress * (1.0 - blend_factor) + quadratic * blend_factor
    } else {
        // Logarithmic curve (more tokens toward start)
        // Use square root approximation: sqrt(progress)
        let sqrt_approx = progress.sqrt();
        // Blend with linear based on how far curve_factor is from 1000
        let steepness = 1.0 - curve_factor_normalized;
        let blend_factor = (steepness * 2.0).min(1.0);
        progress * (1.0 - blend_factor) + sqrt_approx * blend_factor
    };

    // Calculate total amount that should be claimable by now
    let total_claimable = (wallet.total_amount as f64 * curved_progress) as i64;

    // Subtract already claimed amount to get newly claimable amount
    let newly_claimable = total_claimable - wallet.claimed_amount;

    // Make sure we don't exceed remaining balance
    std::cmp::min(newly_claimable, wallet.remaining_balance).max(0)
}
