use crate::link::Link;
use crate::resources::{Asset, LedgerId, OfferId, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::display_fromstr;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Offer {
    #[serde(rename = "_links")]
    pub links: OfferLinks,
    /// The offer id.
    #[serde(with = "display_fromstr")]
    pub id: OfferId,
    /// The paging token for this offer.
    pub paging_token: String,
    /// The account on the sell side.
    pub seller: String,
    /// The asset being sold.
    pub selling: Asset,
    /// The asset being bought.
    pub buying: Asset,
    /// The amount being offered.
    pub amount: String,
    /// The price being offered.
    #[serde(rename = "price_r")]
    pub price_ratio: Price,
    /// The price being offered, as string.
    pub price: String,
    /// The ledger when this offer was last modified.
    pub last_modified_ledger: LedgerId,
    /// The time when this offer was last modified.
    pub last_modified_time: Option<DateTime<Utc>>,
    /// The account sponsoring this offer base reserve.
    pub sponsor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OfferLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    /// Link to the account making the offer.
    pub offer_maker: Link,
}
