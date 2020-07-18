use crate::resources::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ledger {
    #[serde(rename = "_links")]
    pub links: LedgerLinks,
    pub id: String,
    pub paging_token: String,
    pub hash: String,
    #[serde(rename = "prev_hash", skip_serializing_if = "Option::is_none")]
    pub previous_hash: Option<String>,
    pub sequence: i32,
    pub successful_transaction_count: i32,
    pub failed_transaction_count: Option<i32>,
    pub operation_count: i32,
    #[serde(rename = "tx_set_operation_count")]
    pub transaction_set_operation_count: Option<i32>,
    pub closed_at: DateTime<Utc>,
    pub total_coins: String,
    pub fee_pool: String,
    pub base_fee_in_stroops: i32,
    pub base_reserve_in_stroops: i32,
    #[serde(rename = "max_tx_set_size")]
    pub max_transaction_set_size: i32,
    pub protocol_version: i32,
    pub header_xdr: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeDistribution {
    pub max: String,
    pub min: String,
    pub mode: String,
    pub p10: String,
    pub p20: String,
    pub p30: String,
    pub p40: String,
    pub p50: String,
    pub p60: String,
    pub p70: String,
    pub p80: String,
    pub p90: String,
    pub p95: String,
    pub p99: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeStats {
    pub last_ledger: String,
    pub last_ledger_base_fee: String,
    pub ledger_capacity_usage: String,
    pub fee_charged: FeeDistribution,
    pub max_fee: FeeDistribution,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LedgerLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub transactions: Link,
    pub operations: Link,
    pub payments: Link,
    pub effects: Link,
}
