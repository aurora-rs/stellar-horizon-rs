use crate::link::Link;
use crate::resources::{Asset, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Offer {
    #[serde(rename = "_links")]
    pub links: OfferLinks,
    pub id: String,
    pub paging_token: String,
    pub seller: String,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: String,
    #[serde(rename = "price_r")]
    pub price_ratio: Price,
    pub price: String,
    pub last_modified_ledger: i32,
    pub last_modified_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OfferLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub offer_maker: Link,
}
