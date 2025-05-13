// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde_json::json;

// Import event types
use crate::events::my_ip_event_types::{
    LicenseCreatedEvent,
    LicenseUpdatedEvent,
    LicenseTransferredEvent,
    LicenseStateChangedEvent,
    LicenseLinkedEvent,
    LicenseGrantedEvent,
    RevenueDistributedEvent,
};

// Import model types
use crate::models::my_ip::{
    NewMyIP,
    NewMyIPEvent,
    NewMyIPGrant,
    NewMyIPRevenue,
};

// Model conversion impl for LicenseCreatedEvent
impl LicenseCreatedEvent {
    pub fn into_model(&self, transaction_id: String) -> Result<NewMyIP> {
        Ok(NewMyIP {
            license_id: self.license_id.clone(),
            name: self.name.clone(),
            description: Some(self.description.clone()),
            creator: self.creator.clone(),
            creation_time: self.creation_time as i64,
            license_type: self.license_type as i16,
            permission_flags: self.permission_flags as i64,
            license_state: 0, // ACTIVE
            proof_of_creativity_id: self.proof_of_creativity_id.clone(),
            custom_license_uri: self.custom_license_uri.clone(),
            revenue_recipient: self.revenue_recipient.clone(),
            transferable: self.transferable,
            expires_at: self.expires_at.map(|t| t as i64),
            version: 1, // Initial version
            transaction_id,
        })
    }
    
    pub fn into_event(&self, transaction_id: String) -> Result<NewMyIPEvent> {
        let event_data = json!({
            "name": self.name,
            "description": self.description,
            "license_type": self.license_type,
            "permission_flags": self.permission_flags,
            "proof_of_creativity_id": self.proof_of_creativity_id,
            "custom_license_uri": self.custom_license_uri,
            "revenue_recipient": self.revenue_recipient,
            "transferable": self.transferable,
            "expires_at": self.expires_at,
        });
        
        Ok(NewMyIPEvent {
            event_type: "LICENSE_CREATED".to_string(),
            license_id: self.license_id.clone(),
            event_data,
            created_by: self.creator.clone(),
            created_at: self.creation_time as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for LicenseUpdatedEvent
impl LicenseUpdatedEvent {
    pub fn into_event(&self, transaction_id: String) -> Result<NewMyIPEvent> {
        let event_data = json!({
            "updater": self.updater,
            "old_permission_flags": self.old_permission_flags,
            "new_permission_flags": self.new_permission_flags,
        });
        
        Ok(NewMyIPEvent {
            event_type: "LICENSE_UPDATED".to_string(),
            license_id: self.license_id.clone(),
            event_data,
            created_by: self.updater.clone(),
            created_at: self.update_time as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for LicenseTransferredEvent
impl LicenseTransferredEvent {
    pub fn into_event(&self, transaction_id: String) -> Result<NewMyIPEvent> {
        let event_data = json!({
            "from": self.from,
            "to": self.to,
        });
        
        Ok(NewMyIPEvent {
            event_type: "LICENSE_TRANSFERRED".to_string(),
            license_id: self.license_id.clone(),
            event_data,
            created_by: self.from.clone(),
            created_at: self.transfer_time as i64,
            transaction_id,
        })
    }
    
    pub fn into_grant(&self, transaction_id: String) -> Result<NewMyIPGrant> {
        Ok(NewMyIPGrant {
            license_id: self.license_id.clone(),
            grantor: self.from.clone(),
            grantee: self.to.clone(),
            grant_type: "TRANSFER".to_string(),
            payment_amount: 0, // No payment for transfers
            payment_token: None,
            grant_time: self.transfer_time as i64,
            expiration_time: None, // No expiration for transfers
            transaction_id,
        })
    }
}

// Model conversion impl for LicenseStateChangedEvent
impl LicenseStateChangedEvent {
    pub fn into_event(&self, transaction_id: String) -> Result<NewMyIPEvent> {
        let event_data = json!({
            "old_state": self.old_state,
            "new_state": self.new_state,
        });
        
        Ok(NewMyIPEvent {
            event_type: "LICENSE_STATE_CHANGED".to_string(),
            license_id: self.license_id.clone(),
            event_data,
            created_by: self.changer.clone(),
            created_at: self.change_time as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for LicenseLinkedEvent
impl LicenseLinkedEvent {
    pub fn into_event(&self, transaction_id: String) -> Result<NewMyIPEvent> {
        let event_data = json!({
            "post_id": self.post_id,
            "linker": self.linker,
        });
        
        Ok(NewMyIPEvent {
            event_type: "LICENSE_LINKED".to_string(),
            license_id: self.license_id.clone(),
            event_data,
            created_by: self.linker.clone(),
            created_at: self.link_time as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for LicenseGrantedEvent
impl LicenseGrantedEvent {
    pub fn into_event(&self, transaction_id: String) -> Result<NewMyIPEvent> {
        let event_data = json!({
            "ip_id": self.ip_id,
            "grantor": self.grantor,
            "grantee": self.grantee,
            "payment_amount": self.payment_amount,
            "expiration_time": self.expiration_time,
        });
        
        Ok(NewMyIPEvent {
            event_type: "LICENSE_GRANTED".to_string(),
            license_id: self.license_id.clone(),
            event_data,
            created_by: self.grantor.clone(),
            created_at: self.grant_time as i64,
            transaction_id,
        })
    }
    
    pub fn into_grant(&self, transaction_id: String) -> Result<NewMyIPGrant> {
        Ok(NewMyIPGrant {
            license_id: self.license_id.clone(),
            grantor: self.grantor.clone(),
            grantee: self.grantee.clone(),
            grant_type: "LICENSE".to_string(),
            payment_amount: self.payment_amount as i64,
            payment_token: None, // Could be updated if we track token type
            grant_time: self.grant_time as i64,
            expiration_time: self.expiration_time.map(|t| t as i64),
            transaction_id,
        })
    }
}

// Model conversion impl for RevenueDistributedEvent
impl RevenueDistributedEvent {
    pub fn into_event(&self, transaction_id: String) -> Result<NewMyIPEvent> {
        let event_data = json!({
            "post_id": self.post_id,
            "from_address": self.from_address,
            "to_address": self.to_address,
            "amount": self.amount,
            "revenue_type": self.revenue_type,
        });
        
        Ok(NewMyIPEvent {
            event_type: "REVENUE_DISTRIBUTED".to_string(),
            license_id: self.license_id.clone(),
            event_data,
            created_by: self.from_address.clone(),
            created_at: self.distribution_time as i64,
            transaction_id,
        })
    }
    
    pub fn into_revenue(&self, transaction_id: String) -> Result<NewMyIPRevenue> {
        Ok(NewMyIPRevenue {
            license_id: self.license_id.clone(),
            post_id: self.post_id.clone(),
            from_address: self.from_address.clone(),
            to_address: self.to_address.clone(),
            amount: self.amount as i64,
            revenue_type: self.revenue_type.clone(),
            revenue_time: self.distribution_time as i64,
            transaction_id,
        })
    }
} 