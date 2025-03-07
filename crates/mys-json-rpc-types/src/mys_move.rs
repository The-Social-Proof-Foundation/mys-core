// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, DatatypeTyParameter, Visibility};
use move_binary_format::normalized::{
    Enum as NormalizedEnum, Field as NormalizedField, Function as MysNormalizedFunction,
    Module as NormalizedModule, Struct as NormalizedStruct, Type as NormalizedType,
};
use move_core_types::annotated_value::{MoveStruct, MoveValue, MoveVariant};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use mys_macros::EnumVariantOrder;
use tracing::warn;

use mys_types::base_types::{ObjectID, MysAddress};
use mys_types::mys_serde::MysStructTag;

pub type MysMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/mys_move_tests.rs"]
mod mys_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MysMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct MysMoveAbilitySet {
    pub abilities: Vec<MysMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MysMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MysMoveStructTypeParameter {
    pub constraints: MysMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct MysMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: MysMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MysMoveNormalizedStruct {
    pub abilities: MysMoveAbilitySet,
    pub type_parameters: Vec<MysMoveStructTypeParameter>,
    pub fields: Vec<MysMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MysMoveNormalizedEnum {
    pub abilities: MysMoveAbilitySet,
    pub type_parameters: Vec<MysMoveStructTypeParameter>,
    pub variants: BTreeMap<String, Vec<MysMoveNormalizedField>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MysMoveNormalizedType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    #[serde(rename_all = "camelCase")]
    Struct {
        address: String,
        module: String,
        name: String,
        type_arguments: Vec<MysMoveNormalizedType>,
    },
    Vector(Box<MysMoveNormalizedType>),
    TypeParameter(MysMoveTypeParameterIndex),
    Reference(Box<MysMoveNormalizedType>),
    MutableReference(Box<MysMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MysMoveNormalizedFunction {
    pub visibility: MysMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<MysMoveAbilitySet>,
    pub parameters: Vec<MysMoveNormalizedType>,
    pub return_: Vec<MysMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct MysMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MysMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<MysMoveModuleId>,
    pub structs: BTreeMap<String, MysMoveNormalizedStruct>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub enums: BTreeMap<String, MysMoveNormalizedEnum>,
    pub exposed_functions: BTreeMap<String, MysMoveNormalizedFunction>,
}

impl PartialEq for MysMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl From<NormalizedModule> for MysMoveNormalizedModule {
    fn from(module: NormalizedModule) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address.to_hex_literal(),
            name: module.name.to_string(),
            friends: module
                .friends
                .into_iter()
                .map(|module_id| MysMoveModuleId {
                    address: module_id.address().to_hex_literal(),
                    name: module_id.name().to_string(),
                })
                .collect::<Vec<MysMoveModuleId>>(),
            structs: module
                .structs
                .into_iter()
                .map(|(name, struct_)| (name.to_string(), MysMoveNormalizedStruct::from(struct_)))
                .collect::<BTreeMap<String, MysMoveNormalizedStruct>>(),
            enums: module
                .enums
                .into_iter()
                .map(|(name, enum_)| (name.to_string(), MysMoveNormalizedEnum::from(enum_)))
                .collect(),
            exposed_functions: module
                .functions
                .into_iter()
                .filter_map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?
                    (function.is_entry || function.visibility != Visibility::Private)
                        .then(|| (name.to_string(), MysMoveNormalizedFunction::from(function)))
                })
                .collect::<BTreeMap<String, MysMoveNormalizedFunction>>(),
        }
    }
}

impl From<MysNormalizedFunction> for MysMoveNormalizedFunction {
    fn from(function: MysNormalizedFunction) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => MysMoveVisibility::Private,
                Visibility::Public => MysMoveVisibility::Public,
                Visibility::Friend => MysMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<MysMoveAbilitySet>>(),
            parameters: function
                .parameters
                .into_iter()
                .map(MysMoveNormalizedType::from)
                .collect::<Vec<MysMoveNormalizedType>>(),
            return_: function
                .return_
                .into_iter()
                .map(MysMoveNormalizedType::from)
                .collect::<Vec<MysMoveNormalizedType>>(),
        }
    }
}

impl From<NormalizedStruct> for MysMoveNormalizedStruct {
    fn from(struct_: NormalizedStruct) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .into_iter()
                .map(MysMoveStructTypeParameter::from)
                .collect::<Vec<MysMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .into_iter()
                .map(MysMoveNormalizedField::from)
                .collect::<Vec<MysMoveNormalizedField>>(),
        }
    }
}

impl From<NormalizedEnum> for MysMoveNormalizedEnum {
    fn from(value: NormalizedEnum) -> Self {
        Self {
            abilities: value.abilities.into(),
            type_parameters: value
                .type_parameters
                .into_iter()
                .map(MysMoveStructTypeParameter::from)
                .collect::<Vec<MysMoveStructTypeParameter>>(),
            variants: value
                .variants
                .into_iter()
                .map(|variant| {
                    (
                        variant.name.to_string(),
                        variant
                            .fields
                            .into_iter()
                            .map(MysMoveNormalizedField::from)
                            .collect::<Vec<MysMoveNormalizedField>>(),
                    )
                })
                .collect::<BTreeMap<String, Vec<MysMoveNormalizedField>>>(),
        }
    }
}

impl From<DatatypeTyParameter> for MysMoveStructTypeParameter {
    fn from(type_parameter: DatatypeTyParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl From<NormalizedField> for MysMoveNormalizedField {
    fn from(normalized_field: NormalizedField) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: MysMoveNormalizedType::from(normalized_field.type_),
        }
    }
}

impl From<NormalizedType> for MysMoveNormalizedType {
    fn from(type_: NormalizedType) -> Self {
        match type_ {
            NormalizedType::Bool => MysMoveNormalizedType::Bool,
            NormalizedType::U8 => MysMoveNormalizedType::U8,
            NormalizedType::U16 => MysMoveNormalizedType::U16,
            NormalizedType::U32 => MysMoveNormalizedType::U32,
            NormalizedType::U64 => MysMoveNormalizedType::U64,
            NormalizedType::U128 => MysMoveNormalizedType::U128,
            NormalizedType::U256 => MysMoveNormalizedType::U256,
            NormalizedType::Address => MysMoveNormalizedType::Address,
            NormalizedType::Signer => MysMoveNormalizedType::Signer,
            NormalizedType::Struct {
                address,
                module,
                name,
                type_arguments,
            } => MysMoveNormalizedType::Struct {
                address: address.to_hex_literal(),
                module: module.to_string(),
                name: name.to_string(),
                type_arguments: type_arguments
                    .into_iter()
                    .map(MysMoveNormalizedType::from)
                    .collect::<Vec<MysMoveNormalizedType>>(),
            },
            NormalizedType::Vector(v) => {
                MysMoveNormalizedType::Vector(Box::new(MysMoveNormalizedType::from(*v)))
            }
            NormalizedType::TypeParameter(t) => MysMoveNormalizedType::TypeParameter(t),
            NormalizedType::Reference(r) => {
                MysMoveNormalizedType::Reference(Box::new(MysMoveNormalizedType::from(*r)))
            }
            NormalizedType::MutableReference(mr) => {
                MysMoveNormalizedType::MutableReference(Box::new(MysMoveNormalizedType::from(*mr)))
            }
        }
    }
}

impl From<AbilitySet> for MysMoveAbilitySet {
    fn from(set: AbilitySet) -> MysMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => MysMoveAbility::Copy,
                    Ability::Drop => MysMoveAbility::Drop,
                    Ability::Key => MysMoveAbility::Key,
                    Ability::Store => MysMoveAbility::Store,
                })
                .collect::<Vec<MysMoveAbility>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum ObjectValueKind {
    ByImmutableReference,
    ByMutableReference,
    ByValue,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MoveFunctionArgType {
    Pure,
    Object(ObjectValueKind),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveValue")]
pub enum MysMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(MysAddress),
    Vector(Vec<MysMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(MysMoveStruct),
    Option(Box<Option<MysMoveValue>>),
    Variant(MysMoveVariant),
}

impl MysMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            MysMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            MysMoveValue::Vector(values) => MysMoveStruct::Runtime(values).to_json_value(),
            MysMoveValue::Number(v) => json!(v),
            MysMoveValue::Bool(v) => json!(v),
            MysMoveValue::Address(v) => json!(v),
            MysMoveValue::String(v) => json!(v),
            MysMoveValue::UID { id } => json!({ "id": id }),
            MysMoveValue::Option(v) => json!(v),
            MysMoveValue::Variant(v) => v.to_json_value(),
        }
    }
}

impl Display for MysMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            MysMoveValue::Number(value) => write!(writer, "{}", value)?,
            MysMoveValue::Bool(value) => write!(writer, "{}", value)?,
            MysMoveValue::Address(value) => write!(writer, "{}", value)?,
            MysMoveValue::String(value) => write!(writer, "{}", value)?,
            MysMoveValue::UID { id } => write!(writer, "{id}")?,
            MysMoveValue::Struct(value) => write!(writer, "{}", value)?,
            MysMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            MysMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
            MysMoveValue::Variant(value) => write!(writer, "{}", value)?,
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for MysMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => MysMoveValue::Number(value.into()),
            MoveValue::U16(value) => MysMoveValue::Number(value.into()),
            MoveValue::U32(value) => MysMoveValue::Number(value),
            MoveValue::U64(value) => MysMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => MysMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => MysMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => MysMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                MysMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Mys core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                MysMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                MysMoveValue::Address(MysAddress::from(ObjectID::from(value)))
            }
            MoveValue::Variant(MoveVariant {
                type_,
                variant_name,
                tag: _,
                fields,
            }) => MysMoveValue::Variant(MysMoveVariant {
                type_: type_.clone(),
                variant: variant_name.to_string(),
                fields: fields
                    .into_iter()
                    .map(|(id, value)| (id.into_string(), value.into()))
                    .collect::<BTreeMap<_, _>>(),
            }),
        }
    }
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| {
                if let MoveValue::U8(u8) = value {
                    Some(*u8)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "MoveVariant")]
pub struct MysMoveVariant {
    #[schemars(with = "String")]
    #[serde(rename = "type")]
    #[serde_as(as = "MysStructTag")]
    pub type_: StructTag,
    pub variant: String,
    pub fields: BTreeMap<String, MysMoveValue>,
}

impl MysMoveVariant {
    pub fn to_json_value(self) -> Value {
        // We only care about values here, assuming type information is known at the client side.
        let fields = self
            .fields
            .into_iter()
            .map(|(key, value)| (key, value.to_json_value()))
            .collect::<BTreeMap<_, _>>();
        json!({
            "variant": self.variant,
            "fields": fields,
        })
    }
}

impl Display for MysMoveVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        let MysMoveVariant {
            type_,
            variant,
            fields,
        } = self;
        writeln!(writer)?;
        writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
        writeln!(writer, "  {}: {variant}", "variant".bold().bright_black())?;
        for (name, value) in fields {
            let value = format!("{}", value);
            let value = if value.starts_with('\n') {
                indent(&value, 2)
            } else {
                value
            };
            writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
        }

        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveStruct")]
pub enum MysMoveStruct {
    Runtime(Vec<MysMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "MysStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, MysMoveValue>,
    },
    WithFields(BTreeMap<String, MysMoveValue>),
}

impl MysMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            MysMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            MysMoveStruct::WithTypes { type_: _, fields } | MysMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn field_value(&self, field_name: &str) -> Option<MysMoveValue> {
        match self {
            MysMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            MysMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for MysMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            MysMoveStruct::Runtime(_) => {}
            MysMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            MysMoveStruct::WithTypes { type_, fields } => {
                writeln!(writer)?;
                writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
                for (name, value) in fields {
                    let value = format!("{}", value);
                    let value = if value.starts_with('\n') {
                        indent(&value, 2)
                    } else {
                        value
                    };
                    writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
                }
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{}", "", line))
        .join("\n")
}

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<MysMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(MysMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(MysMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(MysMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(MysMoveValue::from);
            if let Some(MysMoveValue::Address(address)) = id {
                return Some(MysMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(MysMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(MysMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(MysMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to MysMoveValue"
    );
    None
}

impl From<MoveStruct> for MysMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        MysMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}
