// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::balance::Balance;
use crate::base_types::MysAddress;
use crate::dynamic_field::{DYNAMIC_FIELD_FIELD_STRUCT_NAME, DYNAMIC_FIELD_MODULE_NAME};
use crate::MYS_FRAMEWORK_ADDRESS;
use crate::MYS_FRAMEWORK_PACKAGE_ID;
use crate::{MoveTypeTagTrait, MoveTypeTagTraitGeneric};
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::{StructTag, TypeTag};
use serde::{Deserialize, Serialize};

pub const ACCUMULATOR_ROOT_MODULE: &IdentStr = ident_str!("accumulator");

const ACCUMULATOR_KEY_TYPE: &IdentStr = ident_str!("Key");
const ACCUMULATOR_U128_TYPE: &IdentStr = ident_str!("U128");

/// Rust type for the Move type accumulator::Key used to derive the dynamic field id for the
/// accumulator value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccumulatorKey {
    pub owner: MysAddress,
}

impl MoveTypeTagTraitGeneric for AccumulatorKey {
    fn get_type_tag(type_params: &[TypeTag]) -> TypeTag {
        TypeTag::Struct(Box::new(StructTag {
            address: MYS_FRAMEWORK_PACKAGE_ID.into(),
            module: ACCUMULATOR_ROOT_MODULE.to_owned(),
            name: ACCUMULATOR_KEY_TYPE.to_owned(),
            type_params: type_params.to_vec(),
        }))
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct U128;

impl MoveTypeTagTrait for U128 {
    fn get_type_tag() -> TypeTag {
        TypeTag::Struct(Box::new(StructTag {
            address: MYS_FRAMEWORK_ADDRESS,
            module: ACCUMULATOR_ROOT_MODULE.to_owned(),
            name: ACCUMULATOR_U128_TYPE.to_owned(),
            type_params: vec![],
        }))
    }
}

use crate::base_types::ObjectID;

/// New-type for ObjectIDs that are known to have been properly derived as a Balance accumulator field.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AccumulatorObjId(ObjectID);

impl AccumulatorObjId {
    pub fn new_unchecked(id: ObjectID) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> &ObjectID {
        &self.0
    }
}

impl std::fmt::Display for AccumulatorObjId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Check if a StructTag represents a balance accumulator field.
/// Returns Some(TypeTag) where TypeTag is the balance type parameter if it is,
/// None otherwise.
pub(crate) fn accumulator_value_balance_type_maybe(s: &StructTag) -> Option<TypeTag> {
    if s.address == MYS_FRAMEWORK_ADDRESS
        && s.module.as_ident_str() == DYNAMIC_FIELD_MODULE_NAME
        && s.name.as_ident_str() == DYNAMIC_FIELD_FIELD_STRUCT_NAME
        && s.type_params.len() == 2
    {
        if let Some(key_type) = accumulator_key_type_maybe(&s.type_params[0]) {
            if is_accumulator_u128(&s.type_params[1]) {
                return Balance::maybe_get_balance_type_param(&key_type);
            }
        }
    }
    None
}

/// Check if a TypeTag is Key<Balance<T>>
pub(crate) fn accumulator_key_type_maybe(t: &TypeTag) -> Option<TypeTag> {
    if let TypeTag::Struct(s) = t {
        if s.address == MYS_FRAMEWORK_ADDRESS
            && s.module.as_ident_str() == ACCUMULATOR_ROOT_MODULE
            && s.name.as_ident_str() == ACCUMULATOR_KEY_TYPE
            && s.type_params.len() == 1
        {
            return Some(s.type_params[0].clone());
        }
    }
    None
}

/// Check if a TypeTag is U128 from accumulator module
pub(crate) fn is_accumulator_u128(t: &TypeTag) -> bool {
    if let TypeTag::Struct(s) = t {
        s.address == MYS_FRAMEWORK_ADDRESS
            && s.module.as_ident_str() == ACCUMULATOR_ROOT_MODULE
            && s.name.as_ident_str() == ACCUMULATOR_U128_TYPE
            && s.type_params.is_empty()
    } else {
        false
    }
}
