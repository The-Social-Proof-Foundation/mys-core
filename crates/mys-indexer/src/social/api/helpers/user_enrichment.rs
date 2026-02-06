// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel_async::RunQueryDsl;
use std::collections::HashMap;

use crate::social::models::universal_user::{
    ReservationStatus, SelectedBadgeInfo, SocialProofTokenInfo, UniversalUserResult,
};

/// Enrich multiple users with universal data in a single efficient batch query
/// Returns a HashMap mapping wallet_address -> UniversalUserResult
pub async fn enrich_users_with_universal_data(
    wallet_addresses: Vec<String>,
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<HashMap<String, UniversalUserResult>, diesel::result::Error> {
    if wallet_addresses.is_empty() {
        return Ok(HashMap::new());
    }

    // Build associated_id values: 'profile_' || owner_address
    let associated_ids: Vec<String> = wallet_addresses
        .iter()
        .map(|addr| format!("profile_{}", addr))
        .collect();

    // Single efficient batch query using CTEs and JOINs
    let query = diesel::sql_query(
        r#"
        WITH latest_profiles AS (
            SELECT DISTINCT ON (COALESCE(profile_id, owner_address)) *
            FROM profiles
            WHERE owner_address = ANY($1::TEXT[])
            ORDER BY COALESCE(profile_id, owner_address), updated_at DESC
        ),
        latest_spt_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_pools
            WHERE associated_id = ANY($2::TEXT[])
            ORDER BY pool_id, time DESC
        ),
        latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            WHERE associated_id = ANY($2::TEXT[])
            ORDER BY pool_id, time DESC
        )
        SELECT 
            p.owner_address,
            p.username,
            p.display_name,
            p.profile_photo,
            p.social_proof_token_address,
            -- Selected badge info
            pb.badge_id as badge_id,
            pb.badge_name,
            pb.badge_icon_url,
            pb.badge_media_url,
            pb.platform_id as badge_platform_id,
            pb.badge_type,
            -- SPT pool info
            spt.pool_id as spt_pool_id,
            -- Reservation pool info
            rp.pool_id as reservation_pool_id,
            rp.total_reserved,
            rp.required_threshold,
            rp.status as reservation_status
        FROM latest_profiles p
        LEFT JOIN profile_badges pb ON 
            p.selected_badge_id IS NOT NULL AND
            pb.badge_id = p.selected_badge_id AND
            pb.profile_id = p.profile_id AND
            pb.revoked = false
        LEFT JOIN latest_spt_pools spt ON
            spt.associated_id = 'profile_' || p.owner_address
        LEFT JOIN latest_reservation_pools rp ON
            rp.associated_id = 'profile_' || p.owner_address
        WHERE p.owner_address = ANY($1::TEXT[])
        "#,
    )
    .bind::<diesel::sql_types::Array<diesel::sql_types::Text>, _>(&wallet_addresses)
    .bind::<diesel::sql_types::Array<diesel::sql_types::Text>, _>(&associated_ids);

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct EnrichmentRow {
        #[diesel(sql_type = Text)]
        owner_address: String,
        #[diesel(sql_type = Text)]
        username: String,
        #[diesel(sql_type = Nullable<Text>)]
        display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_photo: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        social_proof_token_address: Option<String>,
        // Badge fields
        #[diesel(sql_type = Nullable<Text>)]
        badge_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_icon_url: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_media_url: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_platform_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        badge_type: Option<i16>,
        // SPT pool fields
        #[diesel(sql_type = Nullable<Text>)]
        spt_pool_id: Option<String>,
        // Reservation pool fields
        #[diesel(sql_type = Nullable<Text>)]
        reservation_pool_id: Option<String>,
        #[diesel(sql_type = Nullable<BigInt>)]
        total_reserved: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        required_threshold: Option<i64>,
        #[diesel(sql_type = Nullable<Text>)]
        reservation_status: Option<String>,
    }

    let rows: Vec<EnrichmentRow> = query.load::<EnrichmentRow>(conn).await?;

    let mut result = HashMap::new();

    for row in rows {
        // Build selected badge info
        let selected_badge = if let (Some(badge_id), Some(badge_name), Some(platform_id), Some(badge_type)) = (
            row.badge_id.clone(),
            row.badge_name.clone(),
            row.badge_platform_id.clone(),
            row.badge_type,
        ) {
            Some(SelectedBadgeInfo {
                badge_id,
                badge_name,
                badge_icon_url: row.badge_icon_url.clone(),
                badge_media_url: row.badge_media_url.clone(),
                platform_id,
                badge_type,
            })
        } else {
            None
        };

        // Build reservation status
        let reservation_status = match row.reservation_status.as_deref() {
            Some("active") => ReservationStatus::Active,
            Some("threshold_met") => ReservationStatus::ThresholdMet,
            Some("inactive") => ReservationStatus::Inactive,
            _ => ReservationStatus::None,
        };

        // Calculate reservation percentage
        let reservation_percentage = if let (Some(total_reserved), Some(required_threshold)) =
            (row.total_reserved, row.required_threshold)
        {
            if required_threshold > 0 {
                (total_reserved as f64 / required_threshold as f64 * 100.0)
                    .min(100.0)
                    .max(0.0)
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Build social proof token info
        let spt_pool_id = row.spt_pool_id.clone();
        let reservation_pool_id = row.reservation_pool_id.clone();
        let social_proof_token_address = row.social_proof_token_address.clone();
        let is_active = spt_pool_id.is_some();
        
        let social_proof_token = if spt_pool_id.is_some()
            || reservation_pool_id.is_some()
            || social_proof_token_address.is_some()
        {
            Some(SocialProofTokenInfo {
                pool_id: spt_pool_id,
                token_address: social_proof_token_address,
                is_active, // Active if SPT pool exists
                reservation_pool_id,
                reservation_percentage,
                reservation_status: reservation_status.clone(),
                total_reserved: row.total_reserved.unwrap_or(0),
                required_threshold: row.required_threshold.unwrap_or(0),
            })
        } else {
            None
        };

        // Build universal user result
        let user_result = UniversalUserResult {
            wallet_address: row.owner_address.clone(),
            username: Some(row.username),
            fullname: row.display_name,
            profile_photo: row.profile_photo,
            social_proof_token,
            selected_badge,
        };

        result.insert(row.owner_address, user_result);
    }

    // Handle wallet addresses that don't have profiles (wallet-only addresses)
    for wallet_address in wallet_addresses {
        if !result.contains_key(&wallet_address) {
            result.insert(
                wallet_address.clone(),
                UniversalUserResult {
                    wallet_address: wallet_address.clone(),
                    username: None,
                    fullname: None,
                    profile_photo: None,
                    social_proof_token: None,
                    selected_badge: None,
                },
            );
        }
    }

    Ok(result)
}
