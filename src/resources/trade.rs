use crate::link::Link;
use crate::resources::{Asset, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A trade on the distributed exchange.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    /// Trade links.
    #[serde(rename = "_links")]
    pub links: TradeLinks,
    /// A unique identifier for this trade.
    pub id: String,
    /// A cursor value for use in pagination.
    pub paging_token: String,
    /// When the ledger with this trade was closed.
    pub ledger_close_time: DateTime<Utc>,
    /// The sell offer ID.
    pub offer_id: String,
    /// The base offer ID. If this offer was immediately and fully consumed, this will be a synethic ID.
    pub base_offer_id: String,
    /// The account ID of the base party for this trade.
    pub base_account: String,
    /// The amount of the `base_asset` that was moved from `base_account` to `counter_account`.
    pub base_amount: String,
    /// The base asset.
    #[serde(flatten, with = "BaseAsset")]
    pub base_asset: Asset,
    /// The counter offer ID. If this offer was immediately and fully consumed, this will be a synethic ID.
    pub counter_offer_id: String,
    /// The account ID of the counter party for this trade.
    pub counter_account: String,
    /// The amount of the `counter_asset` that was moved from `counter_account` to `base_account`.
    pub counter_amount: String,
    /// The counter asset.
    #[serde(flatten, with = "CounterAsset")]
    pub couter_asset: Asset,
    /// Indicates with party is the seller.
    pub base_is_seller: bool,
    /// The original offer price.
    pub price: Option<Price>,
}

/// Trade effect.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEffect {
    /// Trade effect links.
    #[serde(rename = "_links")]
    pub links: TradeEffectLinks,
    /// A unique identifier for this trade effect.
    pub id: String,
    /// A cursor value for use in pagination.
    pub paging_token: String,
    /// The offer ID.
    pub offer_id: String,
    /// The seller account id.
    pub seller: String,
    /// The amount sold.
    pub sold_amount: String,
    /// The asset sold.
    #[serde(flatten, with = "SoldAsset")]
    pub sold_asset: Asset,
    /// The buyer account id.
    pub buyer: String,
    /// The amount bought.
    pub bought_amount: String,
    /// The asset bougth.
    #[serde(flatten, with = "BoughtAsset")]
    pub bought_asset: Asset,
    /// When the trade effect was created.
    pub created_at: DateTime<Utc>,
}

/// A trade aggregation represents aggregated statistics on an asset pair (base and counter) for a specific time period.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeAggregation {
    /// Start time for this trade aggregation. Represented as milliseconds since epoch.
    pub timestamp: String,
    /// Total number of trades aggregated.
    pub trade_count: String,
    /// Total volume of base asset.
    pub base_volume: String,
    /// Total volume of counter asset.
    pub counter_volume: String,
    /// Weighted average price of counter asset in terms of base asset.
    #[serde(rename = "avg")]
    pub average: String,
    /// The highest price for this time period.
    #[serde(rename = "high_r")]
    pub high_ratio: Price,
    /// The highest price for this time period.
    pub high: String,
    /// The lowest price for this time period.
    #[serde(rename = "low_r")]
    pub low_ratio: Price,
    /// The lowest price for this time period.
    pub low: String,
    /// The price as seen on first trade aggregated.
    #[serde(rename = "open_r")]
    pub open_ratio: Price,
    /// The price as seen on first trade aggregated.
    pub open: String,
    /// The price as seen on last trade aggregated.
    #[serde(rename = "close_r")]
    pub close_ration: Price,
    /// The price as seen on last trade aggregated.
    pub close: String,
}

/// Trade links.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeLinks {
    /// Link to this trade.
    #[serde(rename = "self")]
    pub self_: Link,
    /// Link to base offer.
    pub base: Link,
    /// Link to counter offer.
    pub counter: Link,
    /// Link to operation that triggered the trade.
    pub operation: Link,
}

/// Trade effectt links.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEffectLinks {
    /// Link to this trade effect.
    #[serde(rename = "self")]
    pub self_: Link,
    /// Link to seller.
    pub seller: Link,
    /// Link to buyer.
    pub buyer: Link,
    /// Link to operation that triggered the trade.
    pub operation: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct BaseAsset {
    #[serde(rename = "base_asset_type")]
    asset_type: String,
    #[serde(rename = "base_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "base_asset_issuer")]
    asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct CounterAsset {
    #[serde(rename = "counter_asset_type")]
    asset_type: String,
    #[serde(rename = "counter_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "counter_asset_issuer")]
    asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
pub(crate) struct SoldAsset {
    #[serde(rename = "sold_asset_type")]
    asset_type: String,
    #[serde(rename = "sold_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "sold_asset_issuer")]
    asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
pub(crate) struct BoughtAsset {
    #[serde(rename = "bought_asset_type")]
    asset_type: String,
    #[serde(rename = "bought_asset_code")]
    asset_code: Option<String>,
    #[serde(rename = "bought_asset_issuer")]
    asset_issuer: Option<String>,
}
