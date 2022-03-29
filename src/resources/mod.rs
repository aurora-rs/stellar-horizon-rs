//! Horizon resources.

// All resources have the same type (when possible) of the
// horizon protocol definition at
// https://github.com/stellar/go/blob/master/protocols/horizon/
//
// When updating, use that as your source of truth.
pub mod account;
pub mod asset;
pub mod book;
pub mod claimable_balance;
pub mod effect;
pub mod ledger;
pub mod liquidity_pool;
pub mod offer;
pub mod operation;
pub mod root;
pub mod trade;
pub mod transaction;

pub use account::*;
pub use asset::*;
pub use book::*;
pub use claimable_balance::*;
pub use effect::*;
pub use ledger::*;
pub use liquidity_pool::*;
pub use offer::*;
pub use operation::*;
pub use root::*;
pub use trade::*;
pub use transaction::*;

/// Alias type for offer's id.
pub type OfferId = i64;

/// Alias type for ledger's id.
pub type LedgerId = i32;

/// Alias type for claimable balance id.
pub type ClaimableBalanceId = String;

/// Price as rational number.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Price {
    #[serde(rename = "n")]
    #[serde(alias = "N")]
    pub numerator: i32,
    #[serde(rename = "d")]
    #[serde(alias = "D")]
    pub denominator: i32,
}

/// An asset, either the native asset or a credit asset.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Asset {
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

/// An asset definition or liquidity pool ID.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolOrAsset {
    #[serde(flatten)]
    pub asset: Asset,
    pub liquidity_pool_id: Option<String>,
}

/// An asset along with its amount
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssetAmount {
    pub asset: Option<String>,
    pub amount: String,
}

/// Represent
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Path {
    #[serde(flatten, with = "SourceAsset")]
    pub source_asset: Asset,
    pub source_amount: String,
    #[serde(flatten, with = "DestinationAsset")]
    pub destination_asset: Asset,
    pub destination_amount: String,
    pub path: Vec<Asset>,
}

// https://github.com/serde-rs/serde/issues/970#issuecomment-312282671
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct SourceAsset {
    #[serde(rename = "source_asset_type")]
    asset_type: String,
    #[serde(rename = "source_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "source_asset_issuer")]
    asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct DestinationAsset {
    #[serde(rename = "destination_asset_type")]
    asset_type: String,
    #[serde(rename = "destination_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "destination_asset_issuer")]
    asset_issuer: Option<String>,
}
