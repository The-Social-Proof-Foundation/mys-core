// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use crate::base_types::ObjectID;
use crate::effects::TransactionEffects;
use crate::effects::TransactionEvents;
use crate::error::MysError;
use crate::execution::ExecutionResult;
use crate::full_checkpoint_content::ObjectSet;
use crate::object::Object;
use crate::quorum_driver_types::ExecuteTransactionRequestV3;
use crate::quorum_driver_types::ExecuteTransactionResponseV3;
use crate::quorum_driver_types::QuorumDriverError;
use crate::storage::ObjectKey;
use crate::transaction::TransactionData;

/// Trait to define the interface for how the REST service interacts with a a QuorumDriver or a
/// simulated transaction executor.
#[async_trait::async_trait]
pub trait TransactionExecutor: Send + Sync {
    async fn execute_transaction(
        &self,
        request: ExecuteTransactionRequestV3,
        client_addr: Option<std::net::SocketAddr>,
    ) -> Result<ExecuteTransactionResponseV3, QuorumDriverError>;

    fn simulate_transaction(
        &self,
        transaction: TransactionData,
        checks: TransactionChecks,
        allow_mock_gas_coin: bool,
    ) -> Result<SimulateTransactionResult, MysError>;
}

pub struct SimulateTransactionResult {
    pub effects: TransactionEffects,
    pub events: Option<TransactionEvents>,
    pub objects: ObjectSet,
    pub execution_result: Result<Vec<ExecutionResult>, crate::error::ExecutionError>,
    pub mock_gas_id: Option<ObjectID>,
    pub unchanged_loaded_runtime_objects: Vec<ObjectKey>,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum TransactionChecks {
    #[default]
    Enabled,
    Disabled,
}

impl TransactionChecks {
    pub fn disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    pub fn enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }
}
