// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Import tables from schema
use crate::schema::{my_ip, my_ip_permissions, my_ip_events, my_ip_grants, my_ip_revenue};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, PartialEq)]
#[diesel(table_name = my_ip)]
pub struct MyIP {
    pub id: i32,
    pub license_id: String,
    pub name: String,
    pub description: Option<String>,
    pub creator: String,
    pub creation_time: i64,
    pub license_type: i16,
    pub permission_flags: i64,
    pub license_state: i16,
    pub proof_of_creativity_id: Option<String>,
    pub custom_license_uri: Option<String>,
    pub revenue_recipient: Option<String>,
    pub transferable: bool,
    pub expires_at: Option<i64>,
    pub version: i32,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = my_ip)]
pub struct NewMyIP {
    pub license_id: String,
    pub name: String,
    pub description: Option<String>,
    pub creator: String,
    pub creation_time: i64,
    pub license_type: i16,
    pub permission_flags: i64,
    pub license_state: i16,
    pub proof_of_creativity_id: Option<String>,
    pub custom_license_uri: Option<String>,
    pub revenue_recipient: Option<String>,
    pub transferable: bool,
    pub expires_at: Option<i64>,
    pub version: i32,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = my_ip_permissions)]
pub struct MyIPPermission {
    pub id: i32,
    pub permission_name: String,
    pub bit_position: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = my_ip_events)]
pub struct MyIPEvent {
    pub id: i32,
    pub event_type: String,
    pub license_id: String,
    pub event_data: Value,
    pub created_by: String,
    pub created_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = my_ip_events)]
pub struct NewMyIPEvent {
    pub event_type: String,
    pub license_id: String,
    pub event_data: Value,
    pub created_by: String,
    pub created_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = my_ip_grants)]
pub struct MyIPGrant {
    pub id: i32,
    pub license_id: String,
    pub grantor: String,
    pub grantee: String,
    pub grant_type: String,
    pub payment_amount: i64,
    pub payment_token: Option<String>,
    pub grant_time: i64,
    pub expiration_time: Option<i64>,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = my_ip_grants)]
pub struct NewMyIPGrant {
    pub license_id: String,
    pub grantor: String,
    pub grantee: String,
    pub grant_type: String,
    pub payment_amount: i64,
    pub payment_token: Option<String>,
    pub grant_time: i64,
    pub expiration_time: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = my_ip_revenue)]
pub struct MyIPRevenue {
    pub id: i32,
    pub license_id: String,
    pub post_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub revenue_time: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = my_ip_revenue)]
pub struct NewMyIPRevenue {
    pub license_id: String,
    pub post_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub revenue_time: i64,
    pub transaction_id: String,
}

// Permissions constants for easy reference
pub const PERMISSION_COMMERCIAL_USE: i64 = 1 << 0;
pub const PERMISSION_DERIVATIVES_ALLOWED: i64 = 1 << 1;
pub const PERMISSION_PUBLIC_LICENSE: i64 = 1 << 2;
pub const PERMISSION_AUTHORITY_REQUIRED: i64 = 1 << 3;
pub const PERMISSION_SHARE_ALIKE: i64 = 1 << 4;
pub const PERMISSION_REQUIRE_ATTRIBUTION: i64 = 1 << 5;
pub const PERMISSION_REVENUE_REDIRECT: i64 = 1 << 6;

pub const PERMISSION_ALLOW_COMMENTS: i64 = 1 << 10;
pub const PERMISSION_ALLOW_REACTIONS: i64 = 1 << 11;
pub const PERMISSION_ALLOW_REPOSTS: i64 = 1 << 12;
pub const PERMISSION_ALLOW_QUOTES: i64 = 1 << 13;
pub const PERMISSION_ALLOW_TIPS: i64 = 1 << 14;

// License types
pub const LICENSE_TYPE_CREATIVE_COMMONS: i16 = 0;
pub const LICENSE_TYPE_TOKEN_BOUND: i16 = 1;
pub const LICENSE_TYPE_CUSTOM: i16 = 2;

// License states
pub const LICENSE_STATE_ACTIVE: i16 = 0;
pub const LICENSE_STATE_EXPIRED: i16 = 1;
pub const LICENSE_STATE_REVOKED: i16 = 2;

// Templates for Creative Commons and other common licenses
impl MyIP {
    // Check if a specific permission is granted
    pub fn has_permission(&self, permission: i64) -> bool {
        (self.permission_flags & permission) > 0
    }
    
    // Check specific permissions
    pub fn allows_commercial_use(&self) -> bool {
        self.has_permission(PERMISSION_COMMERCIAL_USE)
    }
    
    pub fn allows_derivatives(&self) -> bool {
        self.has_permission(PERMISSION_DERIVATIVES_ALLOWED)
    }
    
    pub fn allows_comments(&self) -> bool {
        self.has_permission(PERMISSION_ALLOW_COMMENTS)
    }
    
    pub fn allows_reactions(&self) -> bool {
        self.has_permission(PERMISSION_ALLOW_REACTIONS)
    }
    
    pub fn allows_reposts(&self) -> bool {
        self.has_permission(PERMISSION_ALLOW_REPOSTS)
    }
    
    pub fn allows_quotes(&self) -> bool {
        self.has_permission(PERMISSION_ALLOW_QUOTES)
    }
    
    pub fn allows_tips(&self) -> bool {
        self.has_permission(PERMISSION_ALLOW_TIPS)
    }
    
    pub fn requires_attribution(&self) -> bool {
        self.has_permission(PERMISSION_REQUIRE_ATTRIBUTION)
    }
    
    pub fn redirects_revenue(&self) -> bool {
        self.has_permission(PERMISSION_REVENUE_REDIRECT)
    }
    
    // Check if the license is valid (active and not expired)
    pub fn is_valid(&self, current_time: i64) -> bool {
        if self.license_state != LICENSE_STATE_ACTIVE {
            return false;
        }
        
        if let Some(expires_at) = self.expires_at {
            if current_time >= expires_at {
                return false;
            }
        }
        
        true
    }
    
    // Helper methods for creating template-based licenses
    pub fn cc0_permission_flags() -> i64 {
        PERMISSION_COMMERCIAL_USE | 
        PERMISSION_DERIVATIVES_ALLOWED | 
        PERMISSION_PUBLIC_LICENSE |
        PERMISSION_ALLOW_COMMENTS |
        PERMISSION_ALLOW_REACTIONS |
        PERMISSION_ALLOW_REPOSTS |
        PERMISSION_ALLOW_QUOTES |
        PERMISSION_ALLOW_TIPS
    }
    
    pub fn cc_by_permission_flags() -> i64 {
        PERMISSION_COMMERCIAL_USE | 
        PERMISSION_DERIVATIVES_ALLOWED | 
        PERMISSION_PUBLIC_LICENSE | 
        PERMISSION_REQUIRE_ATTRIBUTION |
        PERMISSION_ALLOW_COMMENTS |
        PERMISSION_ALLOW_REACTIONS |
        PERMISSION_ALLOW_REPOSTS |
        PERMISSION_ALLOW_QUOTES |
        PERMISSION_ALLOW_TIPS
    }
}

// Response data type for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyIPWithPostCount {
    #[serde(flatten)]
    pub my_ip: MyIP,
    pub post_count: i64,
    pub content_engagement: i64,
}

// Query result type for SQL queries
#[derive(QueryableByName, Debug, Clone, Serialize, Deserialize)]
pub struct MyIPWithStats {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub license_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = Text)]
    pub creator: String,
    #[diesel(sql_type = BigInt)]
    pub creation_time: i64,
    #[diesel(sql_type = SmallInt)]
    pub license_type: i16,
    #[diesel(sql_type = BigInt)]
    pub permission_flags: i64,
    #[diesel(sql_type = SmallInt)]
    pub license_state: i16,
    #[diesel(sql_type = Nullable<Text>)]
    pub proof_of_creativity_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub custom_license_uri: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revenue_recipient: Option<String>,
    #[diesel(sql_type = Bool)]
    pub transferable: bool,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub expires_at: Option<i64>,
    #[diesel(sql_type = Integer)]
    pub version: i32,
    #[diesel(sql_type = Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = BigInt)]
    pub post_count: i64,
    #[diesel(sql_type = BigInt)]
    pub total_reactions: i64,
    #[diesel(sql_type = BigInt)]
    pub total_comments: i64,
    #[diesel(sql_type = BigInt)]
    pub total_revenue: i64,
}

// Revenue statistics
#[derive(QueryableByName, Debug, Clone, Serialize, Deserialize)]
pub struct RevenueStats {
    #[diesel(sql_type = Text)]
    pub license_id: String,
    #[diesel(sql_type = Text)]
    pub revenue_type: String,
    #[diesel(sql_type = Timestamptz)]
    pub time_bucket: DateTime<Utc>,
    #[diesel(sql_type = BigInt)]
    pub total_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub transaction_count: i64,
} 