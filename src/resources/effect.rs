use crate::link::Link;
use crate::resources::trade::{BoughtAsset, SoldAsset};
use crate::resources::{Asset, AssetAmount};
use crate::resources::{LiquidityPoolOrAsset, Predicate};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::display_fromstr;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    AccountCreated(AccountCreatedEffect),
    AccountRemoved(AccountRemovedEffect),
    AccountCredited(AccountCreditedEffect),
    AccountDebited(AccountDebitedEffect),
    AccountThresholdsUpdated(AccountThresholdsUpdatedEffect),
    AccountHomeDomainUpdated(AccountHomeDomainUpdatedEffect),
    AccountFlagsUpdated(AccountFlagsUpdatedEffect),
    AccountInflationDestinationUpdated(AccountInflationDestinationUpdated),
    SignerCreated(SignerCreatedEffect),
    SignerRemoved(SignerRemovedEffect),
    SignerUpdated(SignerUpdatedEffect),
    #[serde(rename = "trustline_created")]
    TrustLineCreated(TrustLineCreatedEffect),
    #[serde(rename = "trustline_removed")]
    TrustLineRemoved(TrustLineRemovedEffect),
    #[serde(rename = "trustline_updated")]
    TrustLineUpdated(TrustLineUpdatedEffect),
    #[serde(rename = "trustline_authorized")]
    TrustLineAuthorized(TrustLineAuthorizedEffect),
    #[serde(rename = "trustline_authorized_to_maintain_liabilities")]
    TrustLineAuthorizedToMaintainLiabilities(TrustLineAuthorizedToMaintainLiabilitiesEffect),
    #[serde(rename = "trustline_deauthorized")]
    TrustLineDeauthorized(TrustLineDeauthorizedEffect),
    #[serde(rename = "trustline_flags_updated")]
    TrustLineFlagsUpdated(TrustLineFlagsUpdatedEffect),
    Trade(TradeEffect),
    DataCreated(DataCreatedEffect),
    DataRemoved(DataRemovedEffect),
    DataUpdated(DataUpdatedEffect),
    SequenceBumped(SequenceBumpedEffect),
    ClaimableBalanceCreated(ClaimableBalanceCreatedEffect),
    ClaimableBalanceClaimed(ClaimableBalanceClaimedEffect),
    ClaimableBalanceClaimantCreated(ClaimableBalanceClaimantCreatedEffect),
    AccountSponsorshipCreated(AccountSponsorshipCreatedEffect),
    AccountSponsorshipUpdated(AccountSponsorshipUpdatedEffect),
    AccountSponsorshipRemoved(AccountSponsorshipRemovedEffect),
    #[serde(rename = "trustline_sponsorship_created")]
    TrustLineSponsorshipCreated(TrustLineSponsorshipCreatedEffect),
    #[serde(rename = "trustline_sponsorship_updated")]
    TrustLineSponsorshipUpdated(TrustLineSponsorshipUpdatedEffect),
    #[serde(rename = "trustline_sponsorship_removed")]
    TrustLineSponsorshipRemoved(TrustLineSponsorshipRemovedEffect),
    DataSponsorshipCreated(DataSponsorshipCreatedEffect),
    DataSponsorshipUpdated(DataSponsorshipUpdatedEffect),
    DataSponsorshipRemoved(DataSponsorshipRemovedEffect),
    ClaimableBalanceSponsorshipCreated(ClaimableBalanceSponsorshipCreatedEffect),
    ClaimableBalanceSponsorshipUpdated(ClaimableBalanceSponsorshipUpdatedEffect),
    ClaimableBalanceSponsorshipRemoved(ClaimableBalanceSponsorshipRemovedEffect),
    SignerSponsorshipCreated(SignerSponsorshipCreatedEffect),
    SignerSponsorshipUpdated(SignerSponsorshipUpdatedEffect),
    SignerSponsorshipRemoved(SignerSponsorshipRemovedEffect),
    ClaimableBalanceClawedBack(ClaimableBalanceClawedBackEffect),
    LiquidityPoolDeposited(LiquidityPoolDepositedEffect),
    LiquidityPoolWithdrew(LiquidityPoolWithdrewEffect),
    LiquidityPoolTrade(LiquidityPoolTradeEffect),
    LiquidityPoolCreated(LiquidityPoolCreatedEffect),
    LiquidityPoolRemoved(LiquidityPoolRemovedEffect),
    LiquidityPoolRevoked(LiquidityPoolRevokedEffect),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EffectBase {
    #[serde(rename = "_links")]
    pub links: EffectLinks,
    pub id: String,
    pub paging_token: String,
    pub account: String,
    pub account_muxed: Option<String>,
    pub account_muxed_id: Option<String>,
    pub type_i: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub starting_balance: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountCreditedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountDebitedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountThresholdsUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub low_threshold: i32,
    #[serde(rename = "med_threshold")]
    pub medium_threshold: i32,
    pub high_threshold: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountHomeDomainUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub home_domain: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountFlagsUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub auth_required_flag: Option<bool>,
    pub auth_revokable_flag: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountInflationDestinationUpdated {
    #[serde(flatten)]
    pub base: EffectBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SequenceBumpedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    #[serde(rename = "new_seq", with = "display_fromstr")]
    pub new_sequence: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignerCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub weight: i32,
    pub public_key: String,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignerRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub weight: i32,
    pub public_key: String,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignerUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub weight: i32,
    pub public_key: String,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    #[serde(flatten)]
    pub liquidity_pool_or_asset: LiquidityPoolOrAsset,
    pub limit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    #[serde(flatten)]
    pub liquidity_pool_or_asset: LiquidityPoolOrAsset,
    pub limit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    #[serde(flatten)]
    pub liquidity_pool_or_asset: LiquidityPoolOrAsset,
    pub limit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineAuthorizedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub trustor: String,
    pub asset_type: String,
    pub asset_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineAuthorizedToMaintainLiabilitiesEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub trustor: String,
    pub asset_type: String,
    pub asset_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineDeauthorizedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub trustor: String,
    pub asset_type: String,
    pub asset_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineFlagsUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    #[serde(flatten)]
    pub asset: Asset,
    pub trustor: String,
    pub authorized_flag: Option<bool>,
    pub authorized_to_maintain_liabilites_flag: Option<bool>,
    pub clawback_enabled_flag: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OfferCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OfferRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OfferUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DataCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DataRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DataUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TradeEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub seller: String,
    pub seller_muxed: Option<String>,
    pub seller_muxed_id: Option<String>,
    pub offer_id: String,
    pub sold_amount: String,
    #[serde(flatten, with = "SoldAsset")]
    pub sold_asset: Asset,
    pub bought_amount: String,
    #[serde(flatten, with = "BoughtAsset")]
    pub bought_asset: Asset,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub asset: String,
    pub balance_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceClaimedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub asset: String,
    pub balance_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceClaimantCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub asset: String,
    pub balance_id: String,
    pub amount: String,
    pub predicate: Predicate,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountSponsorshipCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountSponsorshipUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub former_sponsor: String,
    pub new_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountSponsorshipRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub former_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineSponsorshipCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub asset_type: String,
    pub asset: Option<String>,
    pub liquidity_pool_id: Option<String>,
    pub sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineSponsorshipUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub asset_type: String,
    pub asset: Option<String>,
    pub liquidity_pool_id: Option<String>,
    pub new_sponsor: String,
    pub former_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLineSponsorshipRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub asset_type: String,
    pub asset: Option<String>,
    pub liquidity_pool_id: Option<String>,
    pub former_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DataSponsorshipCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub data_name: String,
    pub sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DataSponsorshipUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub data_name: String,
    pub former_sponsor: String,
    pub new_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DataSponsorshipRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub data_name: String,
    pub former_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceSponsorshipCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub balance_id: String,
    pub sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceSponsorshipUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub balance_id: String,
    pub former_sponsor: String,
    pub new_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceSponsorshipRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub balance_id: String,
    pub former_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignerSponsorshipCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub signer: String,
    pub sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignerSponsorshipUpdatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub signer: String,
    pub former_sponsor: String,
    pub new_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignerSponsorshipRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub former_sponsor: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceClawedBackEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub balance_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolEffect {
    pub id: String,
    pub fee_bp: u32,
    #[serde(rename = "type")]
    pub pool_type: String,
    #[serde(with = "display_fromstr")]
    pub total_trustlines: u64,
    pub total_shares: String,
    pub reserves: Vec<AssetAmount>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolDepositedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub liquidity_pool: LiquidityPoolEffect,
    pub reserves_deposited: Vec<AssetAmount>,
    pub shares_received: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolWithdrewEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub liquidity_pool: LiquidityPoolEffect,
    pub reserves_received: Vec<AssetAmount>,
    pub shares_redeemed: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolTradeEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub liquidity_pool: LiquidityPoolEffect,
    pub sold: AssetAmount,
    pub bought: AssetAmount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolCreatedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub liquidity_pool: LiquidityPoolEffect,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolRemovedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub liquidity_pool_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolClaimableAssetAmount {
    pub asset: String,
    pub amount: String,
    pub claimable_balance_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolRevokedEffect {
    #[serde(flatten)]
    pub base: EffectBase,
    pub liquidity_pool: LiquidityPoolEffect,
    pub reserves_revoked: Vec<LiquidityPoolClaimableAssetAmount>,
    pub shared_revoked: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EffectLinks {
    pub operation: Link,
    pub succeeds: Link,
    pub precedes: Link,
}

impl Effect {
    pub fn base(&self) -> &EffectBase {
        match self {
            Effect::AccountCreated(op) => &op.base,
            Effect::AccountRemoved(op) => &op.base,
            Effect::AccountCredited(op) => &op.base,
            Effect::AccountDebited(op) => &op.base,
            Effect::AccountThresholdsUpdated(op) => &op.base,
            Effect::AccountHomeDomainUpdated(op) => &op.base,
            Effect::AccountFlagsUpdated(op) => &op.base,
            Effect::AccountInflationDestinationUpdated(op) => &op.base,
            Effect::SignerCreated(op) => &op.base,
            Effect::SignerRemoved(op) => &op.base,
            Effect::SignerUpdated(op) => &op.base,
            Effect::TrustLineCreated(op) => &op.base,
            Effect::TrustLineRemoved(op) => &op.base,
            Effect::TrustLineUpdated(op) => &op.base,
            Effect::TrustLineAuthorized(op) => &op.base,
            Effect::TrustLineAuthorizedToMaintainLiabilities(op) => &op.base,
            Effect::TrustLineDeauthorized(op) => &op.base,
            Effect::TrustLineFlagsUpdated(op) => &op.base,
            Effect::Trade(op) => &op.base,
            Effect::DataCreated(op) => &op.base,
            Effect::DataRemoved(op) => &op.base,
            Effect::DataUpdated(op) => &op.base,
            Effect::SequenceBumped(op) => &op.base,
            Effect::ClaimableBalanceCreated(op) => &op.base,
            Effect::ClaimableBalanceClaimantCreated(op) => &op.base,
            Effect::ClaimableBalanceClaimed(op) => &op.base,
            Effect::AccountSponsorshipCreated(op) => &op.base,
            Effect::AccountSponsorshipUpdated(op) => &op.base,
            Effect::AccountSponsorshipRemoved(op) => &op.base,
            Effect::TrustLineSponsorshipCreated(op) => &op.base,
            Effect::TrustLineSponsorshipUpdated(op) => &op.base,
            Effect::TrustLineSponsorshipRemoved(op) => &op.base,
            Effect::DataSponsorshipCreated(op) => &op.base,
            Effect::DataSponsorshipUpdated(op) => &op.base,
            Effect::DataSponsorshipRemoved(op) => &op.base,
            Effect::ClaimableBalanceSponsorshipCreated(op) => &op.base,
            Effect::ClaimableBalanceSponsorshipUpdated(op) => &op.base,
            Effect::ClaimableBalanceSponsorshipRemoved(op) => &op.base,
            Effect::SignerSponsorshipCreated(op) => &op.base,
            Effect::SignerSponsorshipUpdated(op) => &op.base,
            Effect::SignerSponsorshipRemoved(op) => &op.base,
            Effect::ClaimableBalanceClawedBack(op) => &op.base,
            Effect::LiquidityPoolDeposited(op) => &op.base,
            Effect::LiquidityPoolWithdrew(op) => &op.base,
            Effect::LiquidityPoolTrade(op) => &op.base,
            Effect::LiquidityPoolCreated(op) => &op.base,
            Effect::LiquidityPoolRemoved(op) => &op.base,
            Effect::LiquidityPoolRevoked(op) => &op.base,
        }
    }
}
