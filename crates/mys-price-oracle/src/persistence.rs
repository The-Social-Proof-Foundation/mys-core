use crate::errors::OracleError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleState {
    pub nonce: u64,
    pub last_price: Option<Decimal>,
    pub last_updated: SystemTime,
    pub last_successful_bridge_update: Option<SystemTime>,
}

impl Default for OracleState {
    fn default() -> Self {
        Self {
            nonce: 0,
            last_price: None,
            last_updated: SystemTime::now(),
            last_successful_bridge_update: None,
        }
    }
}

pub struct StateManager {
    db: sled::Db,
}

impl StateManager {
    pub fn new(db_path: &str) -> Result<Self, OracleError> {
        let db = sled::open(db_path)?;
        info!(db_path = %db_path, "Opened state database");
        Ok(Self { db })
    }

    pub fn load_state(&self) -> Result<OracleState, OracleError> {
        match self.db.get(b"oracle_state")? {
            Some(data) => {
                let state: OracleState = serde_json::from_slice(&data)?;
                info!(
                    nonce = state.nonce,
                    last_price = ?state.last_price,
                    "Loaded oracle state from database"
                );
                Ok(state)
            }
            None => {
                info!("No existing state found, using default");
                let state = OracleState::default();
                self.save_state(&state)?;
                Ok(state)
            }
        }
    }

    pub fn save_state(&self, state: &OracleState) -> Result<(), OracleError> {
        let data = serde_json::to_vec(state)?;
        self.db.insert(b"oracle_state", data)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn update_price(&self, price: Decimal) -> Result<(), OracleError> {
        let mut state = self.load_state()?;
        state.last_price = Some(price);
        state.last_updated = SystemTime::now();
        self.save_state(&state)?;
        info!(price = %price, "Updated last price in database");
        Ok(())
    }

    pub fn record_successful_bridge_update(&self) -> Result<(), OracleError> {
        let mut state = self.load_state()?;
        state.last_successful_bridge_update = Some(SystemTime::now());
        self.save_state(&state)?;
        info!("Recorded successful bridge update");
        Ok(())
    }

    pub fn get_next_nonce(&self) -> Result<u64, OracleError> {
        let mut state = self.load_state()?;
        let next_nonce = state.nonce + 1;
        state.nonce = next_nonce;
        state.last_updated = SystemTime::now();
        self.save_state(&state)?;
        Ok(next_nonce)
    }
}
