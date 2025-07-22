// Copyright (c) The Social Proof Foundation LLC
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
                'owner', owner,
                'profile_id', profile_id,
                'has_media', CASE WHEN media_urls IS NOT NULL THEN true ELSE false END
            ) as metadata,
            2 as priority
        FROM posts
        WHERE (
            LOWER(content) LIKE LOWER($1) OR
            LOWER(post_id) LIKE LOWER($1) OR
            LOWER(owner) LIKE LOWER($1) OR
            LOWER(profile_id) LIKE LOWER($1)
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'post' = ANY($4))
        
        UNION ALL
        
        -- Social Proof Token search
        SELECT 
            pool_id::TEXT as id,
            'token' as entity_type,
            name as title,
            description,
            NULL as image_url,
            '/social-proof-token/pools/' || pool_id as url_path,
            symbol as primary_field,
            owner as secondary_field,
            EXTRACT(EPOCH FROM created_at)::BIGINT as timestamp,
            jsonb_build_object(
                'token_type', token_type,
                'base_price', base_price,
                'circulating_supply', circulating_supply,
                'associated_id', associated_id
            ) as metadata,
            3 as priority
        FROM social_proof_token_pools
        WHERE (
            LOWER(pool_id) LIKE LOWER($1) OR
            LOWER(name) LIKE LOWER($1) OR
            LOWER(symbol) LIKE LOWER($1) OR
            LOWER(owner) LIKE LOWER($1) OR
            LOWER(description) LIKE LOWER($1) OR
            LOWER(associated_id) LIKE LOWER($1)
        )
        AND time = (
            SELECT MAX(time) FROM social_proof_token_pools sub
            WHERE sub.pool_id = social_proof_token_pools.pool_id
        )
        AND ($4::TEXT[] IS NULL OR $4 = '{}' OR 'token' = ANY($4))
        
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
                'developer_address', developer_address,
                'is_approved', is_approved,
                'status', status
            ) as metadata,
            4 as priority
        FROM platforms
        WHERE (
            LOWER(platform_id) LIKE LOWER($1) OR
            LOWER(name) LIKE LOWER($1) OR
            LOWER(developer_address) LIKE LOWER($1) OR
            LOWER(description) LIKE LOWER($1)
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
                'submitter', submitter,
                'status', status,
                'community_votes_for', community_votes_for,
                'community_votes_against', community_votes_against
            ) as metadata,
            5 as priority
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
    
    // Count query for pagination
    let count_query = r#"
    WITH combined_results AS (
        -- Profile search
        SELECT 
            'profile' as entity_type,
            COUNT(*) as type_count
        FROM profiles
        WHERE (
            LOWER(owner_address) LIKE LOWER($1) OR
            LOWER(username) LIKE LOWER($1) OR
            LOWER(bio) LIKE LOWER($1)
        )
        AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'profile' = ANY($2))
        
        UNION ALL
        
        -- Post search
        SELECT 
            'post' as entity_type,
            COUNT(*) as type_count
        FROM posts
        WHERE (
            LOWER(content) LIKE LOWER($1) OR
            LOWER(post_id) LIKE LOWER($1) OR
            LOWER(owner) LIKE LOWER($1) OR
            LOWER(profile_id) LIKE LOWER($1)
        )
        AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'post' = ANY($2))
        
        UNION ALL
        
        -- Social Proof Token search
        SELECT 
            'token' as entity_type,
            COUNT(DISTINCT pool_id) as type_count
        FROM social_proof_token_pools
        WHERE (
            LOWER(pool_id) LIKE LOWER($1) OR
            LOWER(name) LIKE LOWER($1) OR
            LOWER(symbol) LIKE LOWER($1) OR
            LOWER(owner) LIKE LOWER($1) OR
            LOWER(description) LIKE LOWER($1) OR
            LOWER(associated_id) LIKE LOWER($1)
        )
        AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'token' = ANY($2))
        
        UNION ALL
        
        -- Platform search
        SELECT 
            'platform' as entity_type,
            COUNT(*) as type_count
        FROM platforms
        WHERE (
            LOWER(platform_id) LIKE LOWER($1) OR
            LOWER(name) LIKE LOWER($1) OR
            LOWER(developer_address) LIKE LOWER($1) OR
            LOWER(description) LIKE LOWER($1)
        )
        AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'platform' = ANY($2))
        
        UNION ALL
        
        -- Governance Proposal search
        SELECT 
            'proposal' as entity_type,
            COUNT(*) as type_count
        FROM proposals
        WHERE (
            LOWER(id) LIKE LOWER($1) OR
            LOWER(title) LIKE LOWER($1) OR
            LOWER(submitter) LIKE LOWER($1) OR
            LOWER(description) LIKE LOWER($1)
        )
        AND ($2::TEXT[] IS NULL OR $2 = '{}' OR 'proposal' = ANY($2))
    )
    SELECT 
        SUM(type_count) as count,
        jsonb_object_agg(entity_type, type_count) as counts_by_type
    FROM combined_results
    "#;
    
    // Get count of search results for pagination
    #[derive(diesel::QueryableByName)]
    struct CountByTypeResult {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        count: i64,
        #[diesel(sql_type = diesel::sql_types::Json)]
        counts_by_type: serde_json::Value,
    }
    
    let count_result = diesel::sql_query(count_query)
        .bind::<diesel::sql_types::Text, _>(&like_query)
        .bind::<diesel::sql_types::Array<diesel::sql_types::Text>, _>(&filter_types)
        .get_result::<CountByTypeResult>(&mut conn)
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
            counts_by_type: count_result.counts_by_type,
        },
        pagination: Some(PaginationInfo {
            page: params.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
} 