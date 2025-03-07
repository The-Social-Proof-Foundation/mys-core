// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::str::FromStr;

use axum::response::{IntoResponse, Response};
use axum::Json;
use fastcrypto::encoding::Hex;
use serde::de::Error as DeError;
use serde::{Deserialize, Serializer};
use serde::{Deserializer, Serialize};
use serde_json::Value;
use strum_macros::EnumIter;
use strum_macros::EnumString;

use mys_sdk::rpc_types::{MysExecutionStatus, MysTransactionBlockKind};
use mys_types::base_types::{ObjectID, ObjectRef, SequenceNumber, MysAddress, TransactionDigest};
use mys_types::crypto::PublicKey as MysPublicKey;
use mys_types::crypto::SignatureScheme;
use mys_types::governance::{ADD_STAKE_FUN_NAME, WITHDRAW_STAKE_FUN_NAME};
use mys_types::messages_checkpoint::CheckpointDigest;
use mys_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use mys_types::mys_system_state::MYS_SYSTEM_MODULE_NAME;
use mys_types::transaction::{Argument, CallArg, Command, ObjectArg, TransactionData};
use mys_types::MYS_SYSTEM_PACKAGE_ID;

use crate::errors::{Error, ErrorType};
use crate::operations::Operations;
use crate::MYS;

#[cfg(test)]
#[path = "unit_tests/types_tests.rs"]
mod types_tests;

pub type BlockHeight = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkIdentifier {
    pub blockchain: String,
    pub network: MysEnv,
}

#[derive(
    Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy, EnumString,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MysEnv {
    MainNet,
    DevNet,
    TestNet,
    LocalNet,
}

impl MysEnv {
    pub fn check_network_identifier(
        &self,
        network_identifier: &NetworkIdentifier,
    ) -> Result<(), Error> {
        if &network_identifier.blockchain != "mys" {
            return Err(Error::UnsupportedBlockchain(
                network_identifier.blockchain.clone(),
            ));
        }
        if &network_identifier.network != self {
            return Err(Error::UnsupportedNetwork(network_identifier.network));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountIdentifier {
    pub address: MysAddress,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<SubAccount>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SubAccount {
    #[serde(rename = "address")]
    pub account_type: SubAccountType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum SubAccountType {
    Stake,
    PendingStake,
    EstimatedReward,
}

impl From<MysAddress> for AccountIdentifier {
    fn from(address: MysAddress) -> Self {
        AccountIdentifier {
            address,
            sub_account: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Currency {
    pub symbol: String,
    pub decimals: u64,
    #[serde(default)]
    pub metadata: CurrencyMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CurrencyMetadata {
    pub coin_type: String,
}

impl Default for CurrencyMetadata {
    fn default() -> Self {
        MYS.metadata.clone()
    }
}

impl Default for Currency {
    fn default() -> Self {
        MYS.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct Currencies(pub Vec<Currency>);

impl Default for Currencies {
    fn default() -> Self {
        Currencies(vec![Currency::default()])
    }
}

fn deserialize_or_default_currencies<'de, D>(deserializer: D) -> Result<Currencies, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Vec<Currency>> = Option::deserialize(deserializer)?;
    match opt {
        Some(vec) if vec.is_empty() => Ok(Currencies::default()),
        Some(vec) => Ok(Currencies(vec)),
        None => Ok(Currencies::default()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    #[serde(default)]
    pub block_identifier: PartialBlockIdentifier,
    #[serde(default, deserialize_with = "deserialize_or_default_currencies")]
    pub currencies: Currencies,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceResponse {
    pub block_identifier: BlockIdentifier,
    pub balances: Vec<Amount>,
}

impl IntoResponse for AccountBalanceResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct BlockIdentifier {
    pub index: BlockHeight,
    pub hash: BlockHash,
}

pub type BlockHash = CheckpointDigest;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Amount {
    #[serde(with = "str_format")]
    pub value: i128,
    #[serde(default)]
    pub currency: Currency,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AmountMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AmountMetadata {
    pub sub_balances: Vec<SubBalance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SubBalance {
    pub stake_id: ObjectID,
    pub validator: MysAddress,
    #[serde(with = "str_format")]
    pub value: i128,
}

impl Amount {
    pub fn new(value: i128, currency: Option<Currency>) -> Self {
        Self {
            value,
            currency: currency.unwrap_or_default(),
            metadata: None,
        }
    }
    pub fn new_from_sub_balances(sub_balances: Vec<SubBalance>) -> Self {
        let value = sub_balances.iter().map(|b| b.value).sum();

        Self {
            value,
            currency: Currency::default(),
            metadata: Some(AmountMetadata { sub_balances }),
        }
    }
}

mod str_format {
    use std::str::FromStr;

    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &i128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i128, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        i128::from_str(&s).map_err(Error::custom)
    }
}

#[derive(Deserialize)]
pub struct AccountCoinsRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    pub include_mempool: bool,
}
#[derive(Serialize)]
pub struct AccountCoinsResponse {
    pub block_identifier: BlockIdentifier,
    pub coins: Vec<Coin>,
}
impl IntoResponse for AccountCoinsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
#[derive(Serialize)]
pub struct Coin {
    pub coin_identifier: CoinIdentifier,
    pub amount: Amount,
}

impl From<mys_sdk::rpc_types::Coin> for Coin {
    fn from(coin: mys_sdk::rpc_types::Coin) -> Self {
        Self {
            coin_identifier: CoinIdentifier {
                identifier: CoinID {
                    id: coin.coin_object_id,
                    version: coin.version,
                },
            },
            amount: Amount {
                value: coin.balance as i128,
                currency: MYS.clone(),
                metadata: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CoinIdentifier {
    pub identifier: CoinID,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoinID {
    pub id: ObjectID,
    pub version: SequenceNumber,
}

impl Serialize for CoinID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{}:{}", self.id, self.version.value()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CoinID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let (id, version) = s.split_at(
            s.find(':')
                .ok_or_else(|| D::Error::custom(format!("Malformed Coin id [{s}].")))?,
        );
        let version = version.trim_start_matches(':');
        let id = ObjectID::from_hex_literal(id).map_err(D::Error::custom)?;
        let version = SequenceNumber::from_u64(u64::from_str(version).map_err(D::Error::custom)?);

        Ok(Self { id, version })
    }
}

#[test]
fn test_coin_id_serde() {
    let id = ObjectID::random();
    let coin_id = CoinID {
        id,
        version: SequenceNumber::from_u64(10),
    };
    let s = serde_json::to_string(&coin_id).unwrap();
    assert_eq!(format!("\"{}:{}\"", id, 10), s);

    let deserialized: CoinID = serde_json::from_str(&s).unwrap();

    assert_eq!(id, deserialized.id);
    assert_eq!(SequenceNumber::from_u64(10), deserialized.version)
}

impl From<ObjectRef> for CoinID {
    fn from((id, version, _): ObjectRef) -> Self {
        Self { id, version }
    }
}

#[derive(Deserialize)]
pub struct MetadataRequest {
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Serialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}

impl IntoResponse for NetworkListResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Deserialize)]
pub struct ConstructionDeriveRequest {
    pub network_identifier: NetworkIdentifier,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub hex_bytes: Hex,
    pub curve_type: CurveType,
}

impl From<MysPublicKey> for PublicKey {
    fn from(pk: MysPublicKey) -> Self {
        match pk {
            MysPublicKey::Ed25519(k) => PublicKey {
                hex_bytes: Hex::from_bytes(&k.0),
                curve_type: CurveType::Edwards25519,
            },
            MysPublicKey::Secp256k1(k) => PublicKey {
                hex_bytes: Hex::from_bytes(&k.0),
                curve_type: CurveType::Secp256k1,
            },
            MysPublicKey::Secp256r1(k) => PublicKey {
                hex_bytes: Hex::from_bytes(&k.0),
                curve_type: CurveType::Secp256r1,
            },
            MysPublicKey::ZkLogin(k) => PublicKey {
                hex_bytes: Hex::from_bytes(&k.0),
                curve_type: CurveType::ZkLogin, // inaccurate but added for completeness.
            },
            MysPublicKey::Passkey(k) => PublicKey {
                hex_bytes: Hex::from_bytes(&k.0),
                curve_type: CurveType::Secp256r1,
            },
        }
    }
}

impl TryInto<MysAddress> for PublicKey {
    type Error = Error;

    fn try_into(self) -> Result<MysAddress, Self::Error> {
        let key_bytes = self.hex_bytes.to_vec()?;
        let pub_key = MysPublicKey::try_from_bytes(self.curve_type.into(), &key_bytes)?;
        Ok((&pub_key).into())
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CurveType {
    Secp256k1,
    Edwards25519,
    Secp256r1,
    ZkLogin,
}

impl From<CurveType> for SignatureScheme {
    fn from(type_: CurveType) -> Self {
        match type_ {
            CurveType::Secp256k1 => SignatureScheme::Secp256k1,
            CurveType::Edwards25519 => SignatureScheme::ED25519,
            CurveType::Secp256r1 => SignatureScheme::Secp256r1,
            CurveType::ZkLogin => SignatureScheme::ZkLoginAuthenticator,
        }
    }
}

#[derive(Serialize)]
pub struct ConstructionDeriveResponse {
    pub account_identifier: AccountIdentifier,
}

impl IntoResponse for ConstructionDeriveResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConstructionPayloadsRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Operations,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConstructionMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<PublicKey>,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, EnumIter, Eq, PartialEq)]
pub enum OperationType {
    // Balance changing operations from TransactionEffect
    Gas,
    MysBalanceChange,
    StakeReward,
    StakePrinciple,
    // mys-rosetta supported operation type
    PayMys,
    PayCoin,
    Stake,
    WithdrawStake,
    // All other Mys transaction types, readonly
    EpochChange,
    Genesis,
    ConsensusCommitPrologue,
    ProgrammableTransaction,
    AuthenticatorStateUpdate,
    RandomnessStateUpdate,
    EndOfEpochTransaction,
}

impl From<&MysTransactionBlockKind> for OperationType {
    fn from(tx: &MysTransactionBlockKind) -> Self {
        match tx {
            MysTransactionBlockKind::ChangeEpoch(_) => OperationType::EpochChange,
            MysTransactionBlockKind::Genesis(_) => OperationType::Genesis,
            MysTransactionBlockKind::ConsensusCommitPrologue(_)
            | MysTransactionBlockKind::ConsensusCommitPrologueV2(_)
            | MysTransactionBlockKind::ConsensusCommitPrologueV3(_) => {
                OperationType::ConsensusCommitPrologue
            }
            MysTransactionBlockKind::ProgrammableTransaction(_) => {
                OperationType::ProgrammableTransaction
            }
            MysTransactionBlockKind::AuthenticatorStateUpdate(_) => {
                OperationType::AuthenticatorStateUpdate
            }
            MysTransactionBlockKind::RandomnessStateUpdate(_) => {
                OperationType::RandomnessStateUpdate
            }
            MysTransactionBlockKind::EndOfEpochTransaction(_) => {
                OperationType::EndOfEpochTransaction
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct OperationIdentifier {
    index: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    network_index: Option<u64>,
}

impl From<u64> for OperationIdentifier {
    fn from(index: u64) -> Self {
        OperationIdentifier {
            index,
            network_index: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct CoinChange {
    pub coin_identifier: CoinIdentifier,
    pub coin_action: CoinAction,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CoinAction {
    CoinCreated,
    CoinSpent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstructionPayloadsResponse {
    pub unsigned_transaction: Hex,
    pub payloads: Vec<SigningPayload>,
}

impl IntoResponse for ConstructionPayloadsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SigningPayload {
    pub account_identifier: AccountIdentifier,
    // Rosetta need the hex string without `0x`, cannot use fastcrypto's Hex
    pub hex_bytes: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignatureType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SignatureType {
    Ed25519,
    Ecdsa,
}

#[derive(Deserialize, Serialize)]
pub struct ConstructionCombineRequest {
    pub network_identifier: NetworkIdentifier,
    pub unsigned_transaction: Hex,
    pub signatures: Vec<Signature>,
}

#[derive(Deserialize, Serialize)]
pub struct Signature {
    pub signing_payload: SigningPayload,
    pub public_key: PublicKey,
    pub signature_type: SignatureType,
    pub hex_bytes: Hex,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstructionCombineResponse {
    pub signed_transaction: Hex,
}

impl IntoResponse for ConstructionCombineResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConstructionSubmitRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: Hex,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionIdentifierResponse {
    pub transaction_identifier: TransactionIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl IntoResponse for TransactionIdentifierResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionIdentifier {
    pub hash: TransactionDigest,
}

#[derive(Serialize, Deserialize)]
pub struct ConstructionPreprocessRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Operations,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PreprocessMetadata>,
}

#[derive(Serialize, Deserialize)]
pub struct PreprocessMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstructionPreprocessResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MetadataOptions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_public_keys: Vec<AccountIdentifier>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataOptions {
    pub internal_operation: InternalOperation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<u64>,
}

impl IntoResponse for ConstructionPreprocessResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
#[derive(Deserialize)]
pub struct ConstructionHashRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: Hex,
}

#[derive(Serialize, Deserialize)]
pub struct ConstructionMetadataRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MetadataOptions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<PublicKey>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstructionMetadataResponse {
    pub metadata: ConstructionMetadata,
    #[serde(default)]
    pub suggested_fee: Vec<Amount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstructionMetadata {
    pub sender: MysAddress,
    pub coins: Vec<ObjectRef>,
    pub objects: Vec<ObjectRef>,
    #[serde(with = "str_format")]
    pub total_coin_value: i128,
    pub gas_price: u64,
    pub budget: u64,
    pub currency: Option<Currency>,
}

impl IntoResponse for ConstructionMetadataResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Deserialize)]
pub struct ConstructionParseRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed: bool,
    pub transaction: Hex,
}

#[derive(Serialize)]
pub struct ConstructionParseResponse {
    pub operations: Operations,
    pub account_identifier_signers: Vec<AccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl IntoResponse for ConstructionParseResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Deserialize)]
pub struct NetworkRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Serialize)]
pub struct NetworkStatusResponse {
    pub current_block_identifier: BlockIdentifier,
    pub current_block_timestamp: u64,
    pub genesis_block_identifier: BlockIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_block_identifier: Option<BlockIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<SyncStatus>,
    pub peers: Vec<Peer>,
}

impl IntoResponse for NetworkStatusResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize)]
pub struct SyncStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_index: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_index: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced: Option<bool>,
}
#[derive(Serialize)]
pub struct Peer {
    pub peer_id: MysAddress,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Serialize)]
pub struct NetworkOptionsResponse {
    pub version: Version,
    pub allow: Allow,
}

impl IntoResponse for NetworkOptionsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize)]
pub struct Version {
    pub rosetta_version: String,
    pub node_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middleware_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Serialize)]
pub struct Allow {
    pub operation_statuses: Vec<Value>,
    pub operation_types: Vec<OperationType>,
    pub errors: Vec<ErrorType>,
    pub historical_balance_lookup: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_start_index: Option<u64>,
    pub call_methods: Vec<String>,
    pub balance_exemptions: Vec<BalanceExemption>,
    pub mempool_coins: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_hash_case: Option<Case>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_hash_case: Option<Case>,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OperationStatus {
    Success,
    Failure,
}

impl From<MysExecutionStatus> for OperationStatus {
    fn from(es: MysExecutionStatus) -> Self {
        match es {
            MysExecutionStatus::Success => OperationStatus::Success,
            MysExecutionStatus::Failure { .. } => OperationStatus::Failure,
        }
    }
}

#[derive(Serialize)]
pub struct BalanceExemption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_account_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exemption_type: Option<ExemptionType>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ExemptionType {
    GreaterOrEqual,
    LessOrEqual,
    Dynamic,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names, dead_code)]
pub enum Case {
    UpperCase,
    LowerCase,
    CaseSensitive,
    Null,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub block_identifier: BlockIdentifier,
    pub parent_block_identifier: BlockIdentifier,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub transaction_identifier: TransactionIdentifier,
    pub operations: Operations,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_transactions: Vec<RelatedTransaction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelatedTransaction {
    network_identifier: NetworkIdentifier,
    transaction_identifier: TransactionIdentifier,
    direction: Direction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockResponse {
    pub block: Block,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_transactions: Vec<TransactionIdentifier>,
}

impl IntoResponse for BlockResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PartialBlockIdentifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<BlockHash>,
}
#[derive(Deserialize)]
pub struct BlockRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(default)]
    pub block_identifier: PartialBlockIdentifier,
}

#[derive(Deserialize)]
pub struct BlockTransactionRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: BlockIdentifier,
    pub transaction_identifier: TransactionIdentifier,
}

#[derive(Serialize)]
pub struct BlockTransactionResponse {
    pub transaction: Transaction,
}

impl IntoResponse for BlockTransactionResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Clone)]
pub struct PrefundedAccount {
    pub privkey: String,
    pub account_identifier: AccountIdentifier,
    pub curve_type: CurveType,
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InternalOperation {
    PayMys {
        sender: MysAddress,
        recipients: Vec<MysAddress>,
        amounts: Vec<u64>,
    },
    PayCoin {
        sender: MysAddress,
        recipients: Vec<MysAddress>,
        amounts: Vec<u64>,
        currency: Currency,
    },
    Stake {
        sender: MysAddress,
        validator: MysAddress,
        amount: Option<u64>,
    },
    WithdrawStake {
        sender: MysAddress,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        stake_ids: Vec<ObjectID>,
    },
}

impl InternalOperation {
    pub fn sender(&self) -> MysAddress {
        match self {
            InternalOperation::PayMys { sender, .. }
            | InternalOperation::PayCoin { sender, .. }
            | InternalOperation::Stake { sender, .. }
            | InternalOperation::WithdrawStake { sender, .. } => *sender,
        }
    }
    /// Combine with ConstructionMetadata to form the TransactionData
    pub fn try_into_data(self, metadata: ConstructionMetadata) -> Result<TransactionData, Error> {
        let pt = match self {
            Self::PayMys {
                recipients,
                amounts,
                ..
            } => {
                let mut builder = ProgrammableTransactionBuilder::new();
                builder.pay_mys(recipients, amounts)?;
                builder.finish()
            }
            Self::PayCoin {
                recipients,
                amounts,
                ..
            } => {
                let mut builder = ProgrammableTransactionBuilder::new();
                builder.pay(metadata.objects.clone(), recipients, amounts)?;
                let currency_str = serde_json::to_string(&metadata.currency.unwrap()).unwrap();
                // This is a workaround in order to have the currency info available during the process
                // of constructing back the Operations object from the transaction data. A process that
                // takes place upon the request to the construction's /parse endpoint. The pure value is
                // not actually being used in any on-chain transaction execution and its sole purpose
                // is to act as a bearer of the currency info between the various steps of the flow.
                // See also the value is being later accessed within the operations.rs file's
                // parse_programmable_transaction function.
                builder.pure(currency_str)?;
                builder.finish()
            }
            InternalOperation::Stake {
                validator, amount, ..
            } => {
                let mut builder = ProgrammableTransactionBuilder::new();

                // [WORKAROUND] - this is a hack to work out if the staking ops is for a selected amount or None amount (whole wallet).
                // if amount is none, validator input will be created after the system object input
                let (validator, system_state, amount) = if let Some(amount) = amount {
                    let amount = builder.pure(amount)?;
                    let validator = builder.input(CallArg::Pure(bcs::to_bytes(&validator)?))?;
                    let state = builder.input(CallArg::MYS_SYSTEM_MUT)?;
                    (validator, state, amount)
                } else {
                    let amount =
                        builder.pure(metadata.total_coin_value as u64 - metadata.budget)?;
                    let state = builder.input(CallArg::MYS_SYSTEM_MUT)?;
                    let validator = builder.input(CallArg::Pure(bcs::to_bytes(&validator)?))?;
                    (validator, state, amount)
                };
                let coin = builder.command(Command::SplitCoins(Argument::GasCoin, vec![amount]));

                let arguments = vec![system_state, coin, validator];

                builder.command(Command::move_call(
                    MYS_SYSTEM_PACKAGE_ID,
                    MYS_SYSTEM_MODULE_NAME.to_owned(),
                    ADD_STAKE_FUN_NAME.to_owned(),
                    vec![],
                    arguments,
                ));
                builder.finish()
            }
            InternalOperation::WithdrawStake { stake_ids, .. } => {
                let mut builder = ProgrammableTransactionBuilder::new();

                for stake_id in metadata.objects {
                    // [WORKAROUND] - this is a hack to work out if the withdraw stake ops is for selected stake_ids or None (all stakes) using the index of the call args.
                    // if stake_ids is not empty, id input will be created after the system object input
                    let (system_state, id) = if !stake_ids.is_empty() {
                        let system_state = builder.input(CallArg::MYS_SYSTEM_MUT)?;
                        let id = builder.obj(ObjectArg::ImmOrOwnedObject(stake_id))?;
                        (system_state, id)
                    } else {
                        let id = builder.obj(ObjectArg::ImmOrOwnedObject(stake_id))?;
                        let system_state = builder.input(CallArg::MYS_SYSTEM_MUT)?;
                        (system_state, id)
                    };

                    let arguments = vec![system_state, id];
                    builder.command(Command::move_call(
                        MYS_SYSTEM_PACKAGE_ID,
                        MYS_SYSTEM_MODULE_NAME.to_owned(),
                        WITHDRAW_STAKE_FUN_NAME.to_owned(),
                        vec![],
                        arguments,
                    ));
                }
                builder.finish()
            }
        };

        Ok(TransactionData::new_programmable(
            metadata.sender,
            metadata.coins,
            pt,
            metadata.budget,
            metadata.gas_price,
        ))
    }
}
