use crate::link::Link;
use crate::resources::{
    Asset, AssetAmount, Claimant, LiquidityPoolOrAsset, Price, SourceAsset, Transaction,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString, DefaultOnNull};
use serde::de::{self, Deserializer};
use serde_json::Value;


#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    CreateAccount(CreateAccountOperation),
    Payment(PaymentOperation),
    PathPaymentStrictReceive(PathPaymentStrictReceiveOperation),
    ManageSellOffer(ManageSellOfferOperation),
    CreatePassiveSellOffer(CreatePassiveSellOfferOperation),
    SetOptions(SetOptionsOperation),
    ChangeTrust(ChangeTrustOperation),
    AllowTrust(AllowTrustOperation),
    AccountMerge(AccountMergeOperation),
    Inflation(InflationOperation),
    ManageData(ManageDataOperation),
    BumpSequence(BumpSequenceOperation),
    ManageBuyOffer(ManageBuyOfferOperation),
    PathPaymentStrictSend(PathPaymentStrictSendOperation),
    CreateClaimableBalance(CreateClaimableBalanceOperation),
    ClaimClaimableBalance(ClaimClaimableBalanceOperation),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOperation),
    EndSponsoringFutureReserves(EndSponsoringFutureReservesOperation),
    RevokeSponsorship(RevokeSponsorshipOperation),
    Clawback(ClawbackOperation),
    ClawbackClaimableBalance(ClawbackClaimableBalanceOperation),
    SetTrustLineFlags(SetTrustLineFlagsOperation),
    LiquidityPoolDeposit(LiquidityPoolDepositOperation),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawOperation),
    InvokeHostFunction(InvokeHostFunctionOperation),
    ExtendFootprintTTL(ExtendFootprintTTLOperation),
    RestoreFootprint(RestoreFootprintOperation),
    Other(OtherOperation), 
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
    InvokeHostFunction(InvokeHostFunctionOperation),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OperationBase {
    #[serde(rename = "_links")]
    pub links: OperationLinks,
    pub id: String,
    pub paging_token: String,
    pub transaction_successful: bool,
    pub source_account: String,
    pub source_account_muxed: Option<String>,
    pub source_account_muxed_id: Option<String>,
    pub type_i: i32,
    pub created_at: DateTime<Utc>,
    pub transaction_hash: String,
    pub transaction: Option<Transaction>,
    pub sponsor: Option<String>,
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
    pub funder_muxed: Option<String>,
    pub funder_muxed_id: Option<String>,
    pub account: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaymentOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub from: String,
    pub from_muxed: Option<String>,
    pub from_muxed_id: Option<String>,
    pub to: String,
    pub to_muxed: Option<String>,
    pub to_muxed_id: Option<String>,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PathPaymentStrictReceiveOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub from: String,
    pub from_muxed: Option<String>,
    pub from_muxed_id: Option<String>,
    pub to: String,
    pub to_muxed: Option<String>,
    pub to_muxed_id: Option<String>,
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
    #[serde(flatten)]
    pub asset: Asset,
    pub from: String,
    pub from_muxed: Option<String>,
    pub from_muxed_id: Option<String>,
    pub to: String,
    pub to_muxed: Option<String>,
    pub to_muxed_id: Option<String>,
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

#[serde_as]
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
    #[serde_as(as = "DisplayFromStr")]
    pub offer_id: i64,
}

#[serde_as]
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
    #[serde_as(as = "DisplayFromStr")]
    pub offer_id: i64,
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
    pub asset_or_pool: LiquidityPoolOrAsset,
    pub limit: String,
    pub trustee: Option<String>,
    pub trustor: String,
    pub trustor_muxed: Option<String>,
    pub trustor_muxed_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AllowTrustOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub trustee: String,
    pub trustee_muxed: Option<String>,
    pub trustee_muxed_id: Option<String>,
    pub trustor: String,
    pub authorize: bool,
    pub authorize_to_maintain_liabilities: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountMergeOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub account: String,
    pub account_muxed: Option<String>,
    pub account_muxed_id: Option<String>,
    pub into: String,
    pub into_muxed: Option<String>,
    pub into_muxed_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InflationOperation {
    #[serde(flatten)]
    pub base: OperationBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateClaimableBalanceOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub asset: String,
    pub amount: String,
    pub claimants: Vec<Claimant>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimClaimableBalanceOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub balance_id: String,
    pub claimant: String,
    pub claimant_muxed: Option<String>,
    pub claimant_muxed_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BeginSponsoringFutureReservesOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub sponsored_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EndSponsoringFutureReservesOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub begin_sponsor: Option<String>,
    pub begin_sponsor_muxed: Option<String>,
    pub begin_sponsor_muxed_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RevokeSponsorshipOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub account_id: Option<String>,
    pub claimable_balance_id: Option<String>,
    pub data_account_id: Option<String>,
    pub data_name: Option<String>,
    pub offer_id: Option<String>,
    pub trustline_account_id: Option<String>,
    pub trustline_liquidity_pool_id: Option<String>,
    pub trustline_asset: Option<String>,
    pub signer_account_id: Option<String>,
    pub signer_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClawbackOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub from: String,
    pub from_muxed: Option<String>,
    pub from_muxed_id: Option<String>,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClawbackClaimableBalanceOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub balance_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SetTrustLineFlagsOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub trustor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub set_flags: Vec<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub set_flags_s: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub clear_flags: Vec<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub clear_flags_s: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolDepositOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub liquidity_pool_id: String,
    pub reserves_max: Vec<AssetAmount>,
    pub min_price: String,
    pub min_price_r: Price,
    pub max_price: String,
    pub max_price_r: Price,
    pub reserves_deposited: Vec<AssetAmount>,
    pub shares_received: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolWithdrawOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub liquidity_pool_id: String,
    pub reserves_min: Vec<AssetAmount>,
    pub shares: String,
    pub reserves_received: Vec<AssetAmount>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum HostFunctionType {
    HostFunctionTypeHostFunctionTypeInvokeContract,
    HostFunctionTypeHostFunctionTypeCreateContract,
    HostFunctionTypeHostFunctionTypeUploadContractWasm,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InvokeContractParameter {
    #[serde(rename = "type")]
    pub type_of: String,
    pub value: String,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InvokeHostFunctionOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub function: HostFunctionType,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parameters: Vec<InvokeContractParameter>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub address: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub salt: Option<String>,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub asset_balance_changes: Vec<AssetBalanceChange>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ExtendFootprintTTLOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    pub extend_to: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RestoreFootprintOperation {
    #[serde(flatten)]
    pub base: OperationBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OtherOperation {
    #[serde(flatten)]
    pub base: OperationBase,
    #[serde(default)]
    pub op_type: String,

    #[serde(default)]
    pub raw_value: serde_json::Value,
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
            Operation::CreateAccount(op) => &op.base,
            Operation::Payment(op) => &op.base,
            Operation::PathPaymentStrictReceive(op) => &op.base,
            Operation::ManageSellOffer(op) => &op.base,
            Operation::CreatePassiveSellOffer(op) => &op.base,
            Operation::SetOptions(op) => &op.base,
            Operation::ChangeTrust(op) => &op.base,
            Operation::AllowTrust(op) => &op.base,
            Operation::AccountMerge(op) => &op.base,
            Operation::Inflation(op) => &op.base,
            Operation::ManageData(op) => &op.base,
            Operation::BumpSequence(op) => &op.base,
            Operation::ManageBuyOffer(op) => &op.base,
            Operation::PathPaymentStrictSend(op) => &op.base,
            Operation::CreateClaimableBalance(op) => &op.base,
            Operation::ClaimClaimableBalance(op) => &op.base,
            Operation::BeginSponsoringFutureReserves(op) => &op.base,
            Operation::EndSponsoringFutureReserves(op) => &op.base,
            Operation::RevokeSponsorship(op) => &op.base,
            Operation::Clawback(op) => &op.base,
            Operation::ClawbackClaimableBalance(op) => &op.base,
            Operation::SetTrustLineFlags(op) => &op.base,
            Operation::LiquidityPoolDeposit(op) => &op.base,
            Operation::LiquidityPoolWithdraw(op) => &op.base,
            Operation::InvokeHostFunction(op) => &op.base,
            Operation::ExtendFootprintTTL(op) => &op.base,
            Operation::RestoreFootprint(op) => &op.base,
            Operation::Other(op) => &op.base,
        }
    }

}

impl<'de> Deserialize<'de> for Operation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;

        let ty: String = v.get("type")
            .and_then(|t| t.as_str())
            .ok_or_else(|| de::Error::missing_field("type"))?
            .to_string();

        fn as_op<T: for<'a> Deserialize<'a>, E: de::Error>(v: &Value) -> Result<T, E> {
            serde_json::from_value::<T>(v.clone()).map_err(E::custom)
        }

        match ty.as_str() {
            "create_account" => Ok(Operation::CreateAccount(as_op(&v)?)),
            "payment" => Ok(Operation::Payment(as_op(&v)?)),
            "path_payment_strict_receive" => Ok(Operation::PathPaymentStrictReceive(as_op(&v)?)),
            "manage_sell_offer" => Ok(Operation::ManageSellOffer(as_op(&v)?)),
            "create_passive_sell_offer" => Ok(Operation::CreatePassiveSellOffer(as_op(&v)?)),
            "set_options" => Ok(Operation::SetOptions(as_op(&v)?)),
            "change_trust" => Ok(Operation::ChangeTrust(as_op(&v)?)),
            "allow_trust" => Ok(Operation::AllowTrust(as_op(&v)?)),
            "account_merge" => Ok(Operation::AccountMerge(as_op(&v)?)),
            "inflation" => Ok(Operation::Inflation(as_op(&v)?)),
            "manage_data" => Ok(Operation::ManageData(as_op(&v)?)),
            "bump_sequence" => Ok(Operation::BumpSequence(as_op(&v)?)),
            "manage_buy_offer" => Ok(Operation::ManageBuyOffer(as_op(&v)?)),
            "path_payment_strict_send" => Ok(Operation::PathPaymentStrictSend(as_op(&v)?)),
            "create_claimable_balance" => Ok(Operation::CreateClaimableBalance(as_op(&v)?)),
            "claim_claimable_balance" => Ok(Operation::ClaimClaimableBalance(as_op(&v)?)),
            "begin_sponsoring_future_reserves" => Ok(Operation::BeginSponsoringFutureReserves(as_op(&v)?)),
            "end_sponsoring_future_reserves" => Ok(Operation::EndSponsoringFutureReserves(as_op(&v)?)),
            "revoke_sponsorship" => Ok(Operation::RevokeSponsorship(as_op(&v)?)),
            "clawback" => Ok(Operation::Clawback(as_op(&v)?)),
            "clawback_claimable_balance" => Ok(Operation::ClawbackClaimableBalance(as_op(&v)?)),
            "set_trust_line_flags" => Ok(Operation::SetTrustLineFlags(as_op(&v)?)),
            "liquidity_pool_deposit" => Ok(Operation::LiquidityPoolDeposit(as_op(&v)?)),
            "liquidity_pool_withdraw" => Ok(Operation::LiquidityPoolWithdraw(as_op(&v)?)),
            "invoke_host_function" => Ok(Operation::InvokeHostFunction(as_op(&v)?)),
            "restore_footprint" => Ok(Operation::RestoreFootprint(as_op(&v)?)),
            "extend_footprint_ttl" => Ok(Operation::ExtendFootprintTTL(as_op(&v)?)),

            _ => {
                let mut other = as_op::<OtherOperation, D::Error>(&v)?;
                other.op_type = ty;
                other.raw_value = v;
                Ok(Operation::Other(other))
            }
        }
    }
}

impl Payment {
    pub fn base(&self) -> &OperationBase {
        match self {
            Payment::CreateAccount(op) => &op.base,
            Payment::Payment(op) => &op.base,
            Payment::PathPaymentStrictReceive(op) => &op.base,
            Payment::PathPaymentStrictSend(op) => &op.base,
            Payment::AccountMerge(op) => &op.base,
            Payment::InvokeHostFunction(op) => &op.base,
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetBalanceChangeType {
    Transfer,
    Mint,
    Clawback,
    Burn,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AssetBalanceChange {
    pub asset_type: String,
    pub code: Option<String>,
    pub issuer: Option<String>,
    #[serde(rename = "type")]
    pub type_of: AssetBalanceChangeType,
    pub from: Option<String>,
    pub to: Option<String>,
    pub amount: String,
}
