use crate::link::Link;
use crate::resources::{Asset, Price, SourceAsset, Transaction};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    BumpSequence(BumpSequenceOperation),
    CreateAccount(CreateAccountOperation),
    Payment(PaymentOperation),
    PathPaymentStrictReceive(PathPaymentStrictReceiveOperation),
    PathPaymentStrictSend(PathPaymentStrictSendOperation),
    ManageData(ManageDataOperation),
    CreatePassiveSellOffer(CreatePassiveSellOfferOperation),
    ManageSellOffer(ManageSellOfferOperation),
    ManageBuyOffer(ManageBuyOfferOperation),
    SetOptions(SetOptionsOperation),
    ChangeTrust(ChangeTrustOperation),
    AllowTrust(AllowTrustOperation),
    AccountMerge(AccountMergeOperation),
    Inflation(InflationOperation),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payment {
    CreateAccount(CreateAccountOperation),
    Payment(PaymentOperation),
    PathPaymentStrictReceive(PathPaymentStrictReceiveOperation),
    PathPaymentStrictSend(PathPaymentStrictSendOperation),
    AccountMerge(AccountMergeOperation),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OperationBase {
    #[serde(rename = "_links")]
    pub links: OperationLinks,
    pub id: String,
    pub paging_token: String,
    pub transaction_successful: bool,
    pub source_account: String,
    pub type_i: i32,
    pub created_at: DateTime<Utc>,
    pub transaction_hash: String,
    pub transaction: Option<Transaction>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BumpSequenceOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub bump_to: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateAccountOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub starting_balance: String,
    pub funder: String,
    pub account: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaymentOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub from: String,
    pub to: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PathPaymentStrictReceiveOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub path: Vec<Asset>,
    pub source_amount: String,
    pub source_max: String,
    #[serde(flatten, with = "SourceAsset")]
    pub source_asset: Asset,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PathPaymentStrictSendOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub path: Vec<Asset>,
    pub source_amount: String,
    pub destination_min: String,
    #[serde(flatten, with = "SourceAsset")]
    pub source_asset: Asset,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ManageDataOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatePassiveSellOfferOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub amount: String,
    pub price: String,
    #[serde(rename = "price_r")]
    pub price_ratio: Price,
    #[serde(flatten, with = "BuyingAsset")]
    pub buying: Asset,
    #[serde(flatten, with = "SellingAsset")]
    pub selling: Asset,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ManageSellOfferOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub amount: String,
    pub price: String,
    #[serde(rename = "price_r")]
    pub price_ratio: Price,
    #[serde(flatten, with = "BuyingAsset")]
    pub buying: Asset,
    #[serde(flatten, with = "SellingAsset")]
    pub selling: Asset,
    pub offer_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ManageBuyOfferOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub amount: String,
    pub price: String,
    #[serde(rename = "price_r")]
    pub price_ratio: Price,
    #[serde(flatten, with = "BuyingAsset")]
    pub buying: Asset,
    #[serde(flatten, with = "SellingAsset")]
    pub selling: Asset,
    pub offer_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SetOptionsOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub home_domain: Option<String>,
    #[serde(rename = "inflation_dest")]
    pub inflation_destination: Option<String>,
    pub master_key_weight: Option<i32>,
    pub signer_key: Option<String>,
    pub signer_weight: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub set_flags: Vec<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub set_flags_s: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub clear_flags: Vec<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub clear_flags_s: Vec<String>,
    pub low_threshold: Option<i32>,
    #[serde(rename = "med_threshold")]
    pub medium_threshold: Option<i32>,
    pub high_threshold: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChangeTrustOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub limit: String,
    pub trustee: String,
    pub trustor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AllowTrustOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub trustee: String,
    pub trustor: String,
    pub authorize: bool,
    pub authorize_to_maintain_liabilities: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountMergeOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub account: String,
    pub into: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InflationOperation {
    #[serde(flatten)]
    pub base: OperationBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OperationLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub transaction: Link,
    pub effects: Link,
    pub succeeds: Link,
    pub precedes: Link,
}

impl Operation {
    pub fn base(&self) -> &OperationBase {
        match self {
            Operation::BumpSequence(op) => &op.base,
            Operation::CreateAccount(op) => &op.base,
            Operation::Payment(op) => &op.base,
            Operation::PathPaymentStrictReceive(op) => &op.base,
            Operation::PathPaymentStrictSend(op) => &op.base,
            Operation::ManageData(op) => &op.base,
            Operation::CreatePassiveSellOffer(op) => &op.base,
            Operation::ManageSellOffer(op) => &op.base,
            Operation::ManageBuyOffer(op) => &op.base,
            Operation::SetOptions(op) => &op.base,
            Operation::ChangeTrust(op) => &op.base,
            Operation::AllowTrust(op) => &op.base,
            Operation::AccountMerge(op) => &op.base,
            Operation::Inflation(op) => &op.base,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct BuyingAsset {
    #[serde(rename = "buying_asset_type")]
    asset_type: String,
    #[serde(rename = "buying_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "buying_asset_issuer")]
    asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct SellingAsset {
    #[serde(rename = "selling_asset_type")]
    asset_type: String,
    #[serde(rename = "selling_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "selling_asset_issuer")]
    asset_issuer: Option<String>,
}
