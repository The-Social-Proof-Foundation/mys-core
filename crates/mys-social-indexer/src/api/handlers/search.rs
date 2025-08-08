// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::error;
use std::sync::Arc;
use diesel_async::RunQueryDsl;

use crate::db::Database;

// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub filter_types: Option<String>, // Comma-separated list of types to include
}

impl SearchParams {
    fn get_page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
    
    fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(20).clamp(1, 100)
    }
    
    fn get_offset(&self) -> i64 {
        (self.get_page() - 1) * self.get_limit()
    }
    
    fn get_filter_types(&self) -> Vec<String> {
        match &self.filter_types {
            Some(types) => types.split(',').map(|s| s.trim().to_string()).collect(),
            None => vec![], // Empty means all types
        }
    }
}

// Pagination info structure
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

// API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub pagination: Option<PaginationInfo>,
}

// Search result item
#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub entity_type: String, 
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub url_path: String,
    pub primary_field: Option<String>,  // Could be address, symbol, username, etc.
    pub secondary_field: Option<String>, // Could be name, title, etc.
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

// Search results
#[derive(Debug, Serialize)]
pub struct SearchResults {
    pub results: Vec<SearchResultItem>,
    pub total_count: i64,
    pub counts_by_type: serde_json::Value,
}

// Common fields for search result rows
#[derive(diesel::QueryableByName)]
struct SearchResultRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub entity_type: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub title: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub image_url: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub url_path: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub primary_field: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub secondary_field: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    pub timestamp: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Json>)]
    pub metadata: Option<serde_json::Value>,
}

/// Global search endpoint that searches across multiple entity types
pub async fn global_search(
    State(db): State<Arc<Database>>,
    Query(params): Query<SearchParams>,
) -> Result<Json<ApiResponse<SearchResults>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let limit = params.get_limit();
    let offset = params.get_offset();
    let search_query = params.query.trim();
    let filter_types = params.get_filter_types();
    
    // Escape the search query for SQL LIKE patterns
    let like_query = format!("%{}%", search_query.replace('%', "\\%").replace('_', "\\_"));
    
    // Build the search query with type filtering logic
    let query_string = r#"
    WITH combined_results AS (
        -- Profile search
        SELECT 
            owner_address::TEXT as id,
            'profile' as entity_type,
            COALESCE(username, 'Anonymous Profile') as title,
            bio as description,
            profile_photo as image_url,
            '/profiles/' || owner_address as url_path,
            username as primary_field,
            owner_address as secondary_field,
            EXTRACT(EPOCH FROM created_at)::BIGINT as timestamp,
            NULL::JSONB as metadata,
            1 as priority
        FROM profiles
        WHERE (
            LOWER(owner_address) LIKE LOWER($1) OR
            LOWER(username) LIKE LOWER($1) OR
            LOWER(bio) LIKE LOWER($1)
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'profile' = ANY($4))
        
        UNION ALL
        
        -- Post search
        SELECT 
            post_id::TEXT as id,
            'post' as entity_type,
            CASE WHEN LENGTH(content) > 50 THEN LEFT(content, 47) || '...' ELSE content END as title,
            content as description,
            NULL as image_url,
            '/posts/' || post_id as url_path,
            NULL as primary_field,
            owner as secondary_field,
            EXTRACT(EPOCH FROM time)::BIGINT as timestamp,
            jsonb_build_object(
                'owner', COALESCE(owner, ''),
                'profile_id', COALESCE(profile_id, ''),
                'has_media', CASE WHEN media_urls IS NOT NULL THEN true ELSE false END
            ) as metadata,
            2 as priority
        FROM posts
        WHERE (
            LOWER(COALESCE(content, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(post_id, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(owner, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(profile_id, '')) LIKE LOWER($1)
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'post' = ANY($4))
        
        UNION ALL
        
        -- Social Proof Token search
        SELECT 
            pool_id::TEXT as id,
            'spt-token' as entity_type,
            name as title,
            NULL as description,
            NULL as image_url,
            '/social-proof-token/pools/' || pool_id as url_path,
            symbol as primary_field,
            owner as secondary_field,
            created_at as timestamp,
            jsonb_build_object(
                'token_type', COALESCE(token_type, 0),
                'base_price', COALESCE(base_price, 0),
                'circulating_supply', COALESCE(circulating_supply, 0),
                'associated_id', COALESCE(associated_id, '')
            ) as metadata,
            3 as priority
        FROM social_proof_token_pools
        WHERE (
            LOWER(COALESCE(pool_id, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(name, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(symbol, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(owner, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(associated_id, '')) LIKE LOWER($1)
        )
        AND time = (
            SELECT MAX(time) FROM social_proof_token_pools sub
            WHERE sub.pool_id = social_proof_token_pools.pool_id
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'spt-token' = ANY($4))
        
        UNION ALL
        
        -- Reservation Pool search
        SELECT 
            pool_id::TEXT as id,
            'spt-reservation-pool' as entity_type,
            CASE 
                WHEN COALESCE(token_type, 0) = 1 THEN 'Profile Reservation Pool: ' || COALESCE(associated_id, '')
                WHEN COALESCE(token_type, 0) = 2 THEN 'Post Reservation Pool: ' || COALESCE(associated_id, '')
                ELSE 'Reservation Pool: ' || COALESCE(pool_id, '')
            END as title,
            CASE 
                WHEN COALESCE(status, '') = 'active' THEN 'Active reservation pool - ' || COALESCE(total_reserved, 0) || '/' || COALESCE(required_threshold, 0) || ' MySo reserved'
                WHEN COALESCE(status, '') = 'threshold_met' THEN 'Threshold met - ready for token creation'
                ELSE 'Reservation pool status: ' || COALESCE(status, '')
            END as description,
            NULL as image_url,
            '/social-proof-token/reservation-pools/' || pool_id as url_path,
            pool_id as primary_field,
            owner as secondary_field,
            created_at as timestamp,
            jsonb_build_object(
                'token_type', COALESCE(token_type, 0),
                'total_reserved', COALESCE(total_reserved, 0),
                'required_threshold', COALESCE(required_threshold, 0),
                'status', COALESCE(status, ''),
                'associated_id', COALESCE(associated_id, ''),
                'progress_percentage', ROUND((COALESCE(total_reserved, 0)::NUMERIC / COALESCE(required_threshold, 1)::NUMERIC) * 100, 2)
            ) as metadata,
            4 as priority
        FROM spt_reservation_pools
        WHERE (
            LOWER(COALESCE(pool_id, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(associated_id, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(owner, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(status, '')) LIKE LOWER($1)
        )
        AND time = (
            SELECT MAX(time) FROM spt_reservation_pools sub
            WHERE sub.pool_id = spt_reservation_pools.pool_id
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'spt-reservation-pool' = ANY($4))
        
        UNION ALL
        
        -- Governance Registry search (Circles)
        SELECT 
            id::TEXT as id,
            'governance-registry' as entity_type,
            CASE 
                WHEN COALESCE(registry_type, 0) = 0 THEN 'Ecosystem Registry'
                WHEN COALESCE(registry_type, 0) = 1 THEN 'Reputation Registry'
                WHEN COALESCE(registry_type, 0) = 2 THEN 'Community Notes Registry'
                ELSE 'Governance Registry #' || COALESCE(registry_type, 0)
            END as title,
            'Governance circle with ' || COALESCE(delegate_count, 0) || ' delegates, ' || COALESCE(proposal_submission_cost, 0) || ' MYS submission cost' as description,
            NULL as image_url,
            '/governance/registries/' || COALESCE(registry_type, 0) as url_path,
            registry_type::TEXT as primary_field,
            delegate_count::TEXT as secondary_field,
            updated_at as timestamp,
            jsonb_build_object(
                'registry_type', COALESCE(registry_type, 0),
                'delegate_count', COALESCE(delegate_count, 0),
                'delegate_term_epochs', COALESCE(delegate_term_epochs, 0),
                'proposal_submission_cost', COALESCE(proposal_submission_cost, 0),
                'voting_period_epochs', COALESCE(voting_period_epochs, 0),
                'quorum_votes', COALESCE(quorum_votes, 0)
            ) as metadata,
            5 as priority
        FROM governance_registries
        WHERE (
            registry_type::TEXT LIKE $1 OR
            delegate_count::TEXT LIKE $1 OR
            LOWER('ecosystem') LIKE LOWER($1) OR
            LOWER('reputation') LIKE LOWER($1) OR
            LOWER('community notes') LIKE LOWER($1) OR
            LOWER('governance') LIKE LOWER($1) OR
            LOWER('registry') LIKE LOWER($1)
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'governance-registry' = ANY($4))
        
        UNION ALL
        
        -- Platform search
        SELECT 
            platform_id::TEXT as id,
            'platform' as entity_type,
            name as title,
            description,
            logo as image_url,
            '/platforms/' || platform_id as url_path,
            platform_id as primary_field,
            developer_address as secondary_field,
            EXTRACT(EPOCH FROM created_at)::BIGINT as timestamp,
            jsonb_build_object(
                'developer_address', COALESCE(developer_address, ''),
                'is_approved', COALESCE(is_approved, false),
                'status', COALESCE(status, 0)
            ) as metadata,
            6 as priority
        FROM platforms
        WHERE (
            LOWER(platform_id) LIKE LOWER($1) OR
            LOWER(name) LIKE LOWER($1) OR
            LOWER(COALESCE(developer_address, '')) LIKE LOWER($1) OR
            LOWER(COALESCE(description, '')) LIKE LOWER($1)
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'platform' = ANY($4))
        
        UNION ALL
        
        -- Governance Proposal search
        SELECT 
            id::TEXT as id,
            'proposal' as entity_type,
            title,
            description,
            NULL as image_url,
            '/governance/proposals/' || id as url_path,
            id as primary_field,
            submitter as secondary_field,
            EXTRACT(EPOCH FROM time)::BIGINT as timestamp,
            jsonb_build_object(
                'submitter', COALESCE(submitter, ''),
                'status', COALESCE(status, 0),
                'community_votes_for', COALESCE(community_votes_for, 0),
                'community_votes_against', COALESCE(community_votes_against, 0)
            ) as metadata,
            7 as priority
        FROM proposals
        WHERE (
            LOWER(id) LIKE LOWER($1) OR
            LOWER(title) LIKE LOWER($1) OR
            LOWER(submitter) LIKE LOWER($1) OR
            LOWER(description) LIKE LOWER($1)
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'proposal' = ANY($4))
    )
    SELECT * FROM combined_results
    -- First exact matches, then partial matches
    ORDER BY 
        CASE WHEN (id = $3 OR primary_field = $3) THEN 0 ELSE 1 END,
        CASE WHEN (
            title ILIKE $3 OR 
            COALESCE(primary_field, '') ILIKE $3 OR
            COALESCE(secondary_field, '') ILIKE $3
        ) THEN 0 ELSE 1 END,
        priority, 
        timestamp DESC NULLS LAST
    LIMIT $2 OFFSET $5
    "#;
    
    // Execute the search query
    let search_results = diesel::sql_query(query_string)
        .bind::<diesel::sql_types::Text, _>(&like_query)
        .bind::<diesel::sql_types::BigInt, _>(limit)
        .bind::<diesel::sql_types::Text, _>(search_query)
        .bind::<diesel::sql_types::Array<diesel::sql_types::Text>, _>(&filter_types)
        .bind::<diesel::sql_types::BigInt, _>(offset)
        .load::<SearchResultRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error in search query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Simplified count query to avoid aggregation issues
    let count_query = r#"
        SELECT COUNT(*) as count
        FROM (
            -- Profile search
            SELECT owner_address as id
            FROM profiles
            WHERE (
                LOWER(owner_address) LIKE LOWER($1) OR
                LOWER(username) LIKE LOWER($1) OR
                LOWER(bio) LIKE LOWER($1)
            )
            AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'profile' = ANY($2))
            
            UNION ALL
            
            -- Post search
            SELECT post_id as id
            FROM posts
            WHERE (
                LOWER(COALESCE(content, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(post_id, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(owner, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(profile_id, '')) LIKE LOWER($1)
            )
            AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'post' = ANY($2))
            
            UNION ALL
            
            -- Social Proof Token search
            SELECT pool_id as id
            FROM social_proof_token_pools
            WHERE (
                LOWER(COALESCE(pool_id, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(name, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(symbol, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(owner, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(associated_id, '')) LIKE LOWER($1)
            )
            AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'spt-token' = ANY($2))
            AND time = (
                SELECT MAX(time) FROM social_proof_token_pools sub
                WHERE sub.pool_id = social_proof_token_pools.pool_id
            )
            
            UNION ALL
            
            -- Reservation Pool search
            SELECT pool_id as id
            FROM spt_reservation_pools
            WHERE (
                LOWER(COALESCE(pool_id, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(associated_id, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(owner, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(status, '')) LIKE LOWER($1)
            )
            AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'spt-reservation-pool' = ANY($2))
            AND time = (
                SELECT MAX(time) FROM spt_reservation_pools sub
                WHERE sub.pool_id = spt_reservation_pools.pool_id
            )
            
            UNION ALL
            
            -- Governance Registry search
            SELECT id::TEXT as id
            FROM governance_registries
            WHERE (
                registry_type::TEXT LIKE $1 OR
                delegate_count::TEXT LIKE $1 OR
                LOWER('ecosystem') LIKE LOWER($1) OR
                LOWER('reputation') LIKE LOWER($1) OR
                LOWER('community notes') LIKE LOWER($1) OR
                LOWER('governance') LIKE LOWER($1) OR
                LOWER('registry') LIKE LOWER($1)
            )
            AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'governance-registry' = ANY($2))
            
            UNION ALL
            
            -- Platform search
            SELECT platform_id as id
            FROM platforms
            WHERE (
                LOWER(platform_id) LIKE LOWER($1) OR
                LOWER(name) LIKE LOWER($1) OR
                LOWER(COALESCE(developer_address, '')) LIKE LOWER($1) OR
                LOWER(COALESCE(description, '')) LIKE LOWER($1)
            )
            AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'platform' = ANY($2))
            
            UNION ALL
            
            -- Governance Proposal search
            SELECT id
            FROM proposals
            WHERE (
                LOWER(id) LIKE LOWER($1) OR
                LOWER(title) LIKE LOWER($1) OR
                LOWER(submitter) LIKE LOWER($1) OR
                LOWER(description) LIKE LOWER($1)
            )
            AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'proposal' = ANY($2))
        ) combined_results
    "#;
    
    // Use the shared CountResult struct
    use crate::db::query_types::CountResult;
    
    let count_result = diesel::sql_query(count_query)
        .bind::<diesel::sql_types::Text, _>(&like_query)
        .bind::<diesel::sql_types::Array<diesel::sql_types::Text>, _>(&filter_types)
        .get_result::<CountResult>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error in count query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Convert query results to SearchResultItem objects
    let results: Vec<SearchResultItem> = search_results
        .into_iter()
        .map(|row| SearchResultItem {
            id: row.id,
            entity_type: row.entity_type,
            title: row.title,
            description: row.description,
            image_url: row.image_url,
            url_path: row.url_path,
            primary_field: row.primary_field,
            secondary_field: row.secondary_field,
            timestamp: row.timestamp,
            metadata: row.metadata,
        })
        .collect();
    
    let total = count_result.count;
    let total_pages = (total + limit - 1) / limit;
    
    Ok(Json(ApiResponse {
        data: SearchResults {
            results,
            total_count: count_result.count,
            counts_by_type: serde_json::json!({}), // Simplified - no longer tracking counts by type
        },
        pagination: Some(PaginationInfo {
            page: params.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
} 