use crate::resources::{Asset, Price};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderBookSummary {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub base: Asset,
    pub counter: Asset,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceLevel {
    #[serde(rename = "price_r")]
    pub price_ratio: Price,
    pub price: String,
    pub amount: String,
}
