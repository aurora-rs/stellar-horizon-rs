use crate::resources::{Asset, Link, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    #[serde(rename = "_links")]
    pub links: TradeLinks,
    pub id: String,
    pub paging_token: String,
    pub ledger_close_time: DateTime<Utc>,
    pub offer_id: String,
    pub base_offer_id: String,
    pub base_account: String,
    pub base_amount: String,
    #[serde(flatten, with = "BaseAsset")]
    pub base_asset: Asset,
    pub counter_offer_id: String,
    pub counter_account: String,
    pub counter_amount: String,
    #[serde(flatten, with = "CounterAsset")]
    pub couter_asset: Asset,
    pub base_is_seller: bool,
    pub price: Option<Price>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEffect {
    #[serde(rename = "_links")]
    pub links: TradeEffectLinks,
    pub id: String,
    pub paging_token: String,
    pub offer_id: String,
    pub seller: String,
    pub sold_amount: String,
    #[serde(flatten, with = "SoldAsset")]
    pub sold_asset: Asset,
    pub buyer: String,
    pub bought_amount: String,
    #[serde(flatten, with = "BoughtAsset")]
    pub bought_asset: Asset,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeAggregation {
    pub timestamp: String,
    pub trade_count: String,
    pub base_volume: String,
    pub counter_volume: String,
    #[serde(rename = "avg")]
    pub average: String,
    #[serde(rename = "high_r")]
    pub high_ratio: Price,
    pub high: String,
    #[serde(rename = "low_r")]
    pub low_ratio: Price,
    pub low: String,
    #[serde(rename = "open_r")]
    pub open_ratio: Price,
    pub open: String,
    #[serde(rename = "close_r")]
    pub close_ration: Price,
    pub close: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub base: Link,
    pub counter: Link,
    pub operation: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeEffectLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub seller: Link,
    pub buyer: Link,
    pub operation: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct BaseAsset {
    #[serde(rename = "base_asset_type")]
    pub asset_type: String,
    #[serde(rename = "base_asset_code")]
    pub asset_code: Option<String>,
    #[serde(rename = "base_asset_issuer")]
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct CounterAsset {
    #[serde(rename = "counter_asset_type")]
    pub asset_type: String,
    #[serde(rename = "counter_asset_code")]
    pub asset_code: Option<String>,
    #[serde(rename = "counter_asset_issuer")]
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct SoldAsset {
    #[serde(rename = "sold_asset_type")]
    pub asset_type: String,
    #[serde(rename = "sold_asset_code")]
    pub asset_code: Option<String>,
    #[serde(rename = "sold_asset_issuer")]
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Asset")]
struct BoughtAsset {
    #[serde(rename = "bought_asset_type")]
    pub asset_type: String,
    #[serde(rename = "bought_asset_code")]
    pub asset_code: Option<String>,
    #[serde(rename = "bought_asset_issuer")]
    pub asset_issuer: Option<String>,
}
