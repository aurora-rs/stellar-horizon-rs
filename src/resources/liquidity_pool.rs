use crate::link::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::display_fromstr;

/// Liquidity Pool on the Stellar Network.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPool {
    /// Liquidity Pool links.
    #[serde(rename = "_links")]
    pub links: LiquidityPoolLinks,
    /// An unique identifier for this liquidity pool.
    pub id: String,
    /// The cursor value.
    pub paging_token: String,
    /// The fee the liquidity pool charges per transaction in basis points.
    pub fee_bp: u32,
    /// The liqudity pool type.
    #[serde(rename = "type")]
    pub pool_type: String,
    /// The number of accounts that have a trustline for this liquidity pool.
    #[serde(with = "display_fromstr")]
    pub total_trustlines: u64,
    /// The number of outstanding shares of the liquidity pool.
    pub total_shares: String,
    /// The assets contained in the liquidity pool.
    pub reserves: Vec<LiquidityPoolReserve>,
    /// The id of the last ledger where this liquidity pool had activity.
    pub last_modified_ledger: u32,
    /// The time when this liquidity pool was last modified.
    pub last_modified_time: Option<DateTime<Utc>>,
}

/// Links for a liquidity pool.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolLinks {
    /// This liquidity pool.
    #[serde(rename = "self")]
    pub self_: Link,
    /// This liquidity pool's transactions.
    pub transactions: Link,
    /// This liquidity pool's operation.
    pub operations: Link,
}

/// Asset reserve in a liquidity pool.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiquidityPoolReserve {
    /// The asset held in this liquidity pool reserve.
    pub asset: String,
    /// The balance of this asset in the liquidity pool.
    pub amount: String,
}
