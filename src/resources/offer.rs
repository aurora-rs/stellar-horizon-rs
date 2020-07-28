use crate::link::Link;
use crate::resources::{Asset, LedgerId, OfferId, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::display_fromstr;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Offer {
    #[serde(rename = "_links")]
    pub links: OfferLinks,
    #[serde(with = "display_fromstr")]
    pub id: OfferId,
    pub paging_token: String,
    pub seller: String,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: String,
    #[serde(rename = "price_r")]
    pub price_ratio: Price,
    pub price: String,
    pub last_modified_ledger: LedgerId,
    pub last_modified_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OfferLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub offer_maker: Link,
}
