// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::bail;
use move_core_types::language_storage::TypeTag;
use mys_json_rpc_types::{BalanceChange, MysData, MysObjectData, MysObjectDataOptions};
use mys_sdk::MysClient;
use mys_types::error::MysObjectResponseError;
use mys_types::gas_coin::GasCoin;
use mys_types::{base_types::ObjectID, object::Owner, parse_mys_type_tag};
use tracing::{debug, trace};

/// A util struct that helps verify Mys Object.
/// Use builder style to construct the conditions.
/// When optionals fields are not set, related checks are omitted.
/// Consuming functions such as `check` perform the check and panics if
/// verification results are unexpected. `check_into_object` and
/// `check_into_gas_coin` expect to get a `MysObjectData` and `GasCoin`
/// respectfully.
#[derive(Debug)]
pub struct ObjectChecker {
    object_id: ObjectID,
    owner: Option<Owner>,
    is_deleted: bool,
    is_mys_coin: Option<bool>,
}

impl ObjectChecker {
    pub fn new(object_id: ObjectID) -> ObjectChecker {
        Self {
            object_id,
            owner: None,
            is_deleted: false, // default to exist
            is_mys_coin: None,
        }
    }

    pub fn owner(mut self, owner: Owner) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn deleted(mut self) -> Self {
        self.is_deleted = true;
        self
    }

    pub fn is_mys_coin(mut self, is_mys_coin: bool) -> Self {
        self.is_mys_coin = Some(is_mys_coin);
        self
    }

    pub async fn check_into_gas_coin(self, client: &MysClient) -> GasCoin {
        if self.is_mys_coin == Some(false) {
            panic!("'check_into_gas_coin' shouldn't be called with 'is_mys_coin' set as false");
        }
        self.is_mys_coin(true)
            .check(client)
            .await
            .unwrap()
            .into_gas_coin()
    }

    pub async fn check_into_object(self, client: &MysClient) -> MysObjectData {
        self.check(client).await.unwrap().into_object()
    }

    pub async fn check(self, client: &MysClient) -> Result<CheckerResultObject, anyhow::Error> {
        debug!(?self);

        let object_id = self.object_id;
        let object_info = client
            .read_api()
            .get_object_with_options(
                object_id,
                MysObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_bcs(),
            )
            .await
            .or_else(|err| bail!("Failed to get object info (id: {}), err: {err}", object_id))?;

        trace!("getting object {object_id}, info :: {object_info:?}");

        match (object_info.data, object_info.error) {
            (None, Some(MysObjectResponseError::NotExists { object_id })) => {
                panic!(
                    "Node can't find gas object {} with client {:?}",
                    object_id,
                    client.read_api()
                )
            }
            (
                None,
                Some(MysObjectResponseError::DynamicFieldNotFound {
                    parent_object_id: object_id,
                }),
            ) => {
                panic!(
                    "Node can't find dynamic field for {} with client {:?}",
                    object_id,
                    client.read_api()
                )
            }
            (
                None,
                Some(MysObjectResponseError::Deleted {
                    object_id,
                    version: _,
                    digest: _,
                }),
            ) => {
                if !self.is_deleted {
                    panic!("Gas object {} was deleted", object_id);
                }
                Ok(CheckerResultObject::new(None, None))
            }
            (Some(object), _) => {
                if self.is_deleted {
                    panic!("Expect Gas object {} deleted, but it is not", object_id);
                }
                if let Some(owner) = self.owner {
                    let object_owner = object
                        .owner
                        .clone()
                        .unwrap_or_else(|| panic!("Object {} does not have owner", object_id));
                    assert_eq!(
                        object_owner, owner,
                        "Gas coin {} does not belong to {}, but {}",
                        object_id, owner, object_owner
                    );
                }
                if self.is_mys_coin == Some(true) {
                    let move_obj = object
                        .bcs
                        .as_ref()
                        .unwrap_or_else(|| panic!("Object {} does not have bcs data", object_id))
                        .try_as_move()
                        .unwrap_or_else(|| panic!("Object {} is not a move object", object_id));

                    let gas_coin = move_obj.deserialize()?;
                    return Ok(CheckerResultObject::new(Some(gas_coin), Some(object)));
                }
                Ok(CheckerResultObject::new(None, Some(object)))
            }
            (None, Some(MysObjectResponseError::DisplayError { error })) => {
                panic!("Display Error: {error:?}");
            }
            (None, None) | (None, Some(MysObjectResponseError::Unknown)) => {
                panic!("Unexpected response: object not found and no specific error provided");
            }
        }
    }
}

pub struct CheckerResultObject {
    gas_coin: Option<GasCoin>,
    object: Option<MysObjectData>,
}

impl CheckerResultObject {
    pub fn new(gas_coin: Option<GasCoin>, object: Option<MysObjectData>) -> Self {
        Self { gas_coin, object }
    }
    pub fn into_gas_coin(self) -> GasCoin {
        self.gas_coin.unwrap()
    }
    pub fn into_object(self) -> MysObjectData {
        self.object.unwrap()
    }
}

#[macro_export]
macro_rules! assert_eq_if_present {
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (Some(left_val), right_val) => {
                 if !(&left_val == right_val) {
                    panic!("{} does not match, left: {:?}, right: {:?}", $($arg)+, left_val, right_val);
                }
            }
            _ => ()
        }
    };
}

#[derive(Default, Debug)]
pub struct BalanceChangeChecker {
    owner: Option<Owner>,
    coin_type: Option<TypeTag>,
    amount: Option<i128>,
}

impl BalanceChangeChecker {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn owner(mut self, owner: Owner) -> Self {
        self.owner = Some(owner);
        self
    }
    pub fn coin_type(mut self, coin_type: &str) -> Self {
        self.coin_type = Some(parse_mys_type_tag(coin_type).unwrap());
        self
    }

    pub fn amount(mut self, amount: i128) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn check(self, event: &BalanceChange) {
        let BalanceChange {
            owner,
            coin_type,
            amount,
        } = event;

        assert_eq_if_present!(self.owner, owner, "owner");
        assert_eq_if_present!(self.coin_type, coin_type, "coin_type");
        assert_eq_if_present!(self.amount, amount, "version");
    }
}
