use crate::link::Link;
use crate::resources::Asset;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A trade on the distributed exchange.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
    pub offer_id: Option<String>,
    /// The type of trade that was executed.
    pub trade_type: String,
    /// The fee that was charged by the liquidity pool in basis points.
    pub liquidity_pool_fee_bp: Option<u32>,
    /// The ID for the liquidity pool that acted as the base party.
    pub base_liquidity_pool_id: Option<String>,
    /// The base offer ID. If this offer was immediately and fully consumed, this will be a synthetic ID.
    pub base_offer_id: Option<String>,
    /// The account ID of the base party for this trade.
    pub base_account: Option<String>,
    /// The amount of the `base_asset` that was moved from `base_account` to `counter_account`.
    pub base_amount: String,
    /// The base asset.
    #[serde(flatten, with = "BaseAsset")]
    pub base_asset: Asset,
    /// The ID for the liquidity pool that acted as the counter party.
    pub counter_liquidity_pool_id: Option<String>,
    /// The counter offer ID. If this offer was immediately and fully consumed, this will be a synthetic ID.
    pub counter_offer_id: Option<String>,
    /// The account ID of the counter party for this trade.
    pub counter_account: Option<String>,
    /// The amount of the `counter_asset` that was moved from `counter_account` to `base_account`.
    pub counter_amount: String,
    /// The counter asset.
    #[serde(flatten, with = "CounterAsset")]
    pub counter_asset: Asset,
    /// Indicates with party is the seller.
    pub base_is_seller: bool,
    /// The original offer price.
    pub price: Option<TradePrice>,
}

/// Price for a trade
#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TradePrice {
    /// The numerator of the price
    #[serde(rename = "n")]
    #[serde_as(as = "DisplayFromStr")]
    pub numerator: i64,
    /// The denominator of the price
    #[serde(rename = "d")]
    #[serde_as(as = "DisplayFromStr")]
    pub denominator: i64,
}

/// A trade aggregation represents aggregated statistics on an asset pair (base and counter) for a specific time period.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TradeAggregation {
    /// Start time for this trade aggregation. Represented as milliseconds since epoch.
    #[serde_as(as = "DisplayFromStr")]
    pub timestamp: i64,
    /// Total number of trades aggregated.
    #[serde_as(as = "DisplayFromStr")]
    pub trade_count: i64,
    /// Total volume of base asset.
    pub base_volume: String,
    /// Total volume of counter asset.
    pub counter_volume: String,
    /// Weighted average price of counter asset in terms of base asset.
    #[serde(rename = "avg")]
    pub average: String,
    /// The highest price for this time period.
    pub high: String,
    /// The highest price for this time period.
    #[serde(rename = "high_r")]
    pub high_ratio: TradePrice,
    /// The lowest price for this time period.
    pub low: String,
    /// The lowest price for this time period.
    #[serde(rename = "low_r")]
    pub low_ratio: TradePrice,
    /// The price as seen on first trade aggregated.
    pub open: String,
    /// The price as seen on first trade aggregated.
    #[serde(rename = "open_r")]
    pub open_ratio: TradePrice,
    /// The price as seen on last trade aggregated.
    pub close: String,
    /// The price as seen on last trade aggregated.
    #[serde(rename = "close_r")]
    pub close_ratio: TradePrice,
}

/// Trade links.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
