use crate::link::Link;
use crate::resources::LedgerId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::display_fromstr;

/// Store the state of network at a point in time.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Ledger {
    /// HAL links.
    #[serde(rename = "_links")]
    pub links: LedgerLinks,
    /// An unique identifier.
    pub id: String,
    /// The cursor value.
    pub paging_token: String,
    /// A hex-encoded SHA-256 hash of this ledger XDR encoded form.
    pub hash: String,
    /// The hash of the ledger preceding this one.
    #[serde(rename = "prev_hash", skip_serializing_if = "Option::is_none")]
    pub previous_hash: Option<String>,
    /// The ledger sequence number.
    pub sequence: i32,
    /// The number of successfull transactions in this ledger.
    pub successful_transaction_count: i32,
    /// The number of failed transactions in this ledger.
    pub failed_transaction_count: Option<i32>,
    /// The number of operations applied in this ledger.
    pub operation_count: i32,
    /// The number of operations in the transaction set.
    #[serde(rename = "tx_set_operation_count")]
    pub transaction_set_operation_count: Option<i32>,
    /// When this ledger was closed.
    pub closed_at: DateTime<Utc>,
    /// Total number of lumens in circulation.
    pub total_coins: String,
    /// The sum of all transaction fees.
    pub fee_pool: String,
    /// The fee the network charges per operation.
    pub base_fee_in_stroops: i32,
    /// The reserve the network uses when calculating the minimum balance.
    pub base_reserve_in_stroops: i32,
    /// The maximum number of transactions validators have agreed to process in a ledger.
    #[serde(rename = "max_tx_set_size")]
    pub max_transaction_set_size: i32,
    /// The protocol version the network was running when this transaction was closed.
    pub protocol_version: i32,
    /// An base64 encoded string of the raw `LedgerHeader` xdr structure for this ledger.
    pub header_xdr: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FeeDistribution {
    /// Maximum fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub max: i64,
    /// Minimum fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub min: i64,
    /// Mode fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub mode: i64,
    /// 10th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p10: i64,
    /// 20th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p20: i64,
    /// 30th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p30: i64,
    /// 40th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p40: i64,
    /// 50th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p50: i64,
    /// 60th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p60: i64,
    /// 70th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p70: i64,
    /// 80th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p80: i64,
    /// 90th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p90: i64,
    /// 95th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p95: i64,
    /// 99th percentile fee charged over the last 5 ledger.
    #[serde(with = "display_fromstr")]
    pub p99: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FeeStats {
    /// The last ledger sequence number.
    #[serde(with = "display_fromstr")]
    pub last_ledger: LedgerId,
    /// The last ledger base fee.
    #[serde(with = "display_fromstr")]
    pub last_ledger_base_fee: i64,
    /// The average capacity usage over the last 5 ledgers (0 is no usage, 1 is complete usage).
    #[serde(with = "display_fromstr")]
    pub ledger_capacity_usage: f64,
    /// Information about the fee charged.
    pub fee_charged: FeeDistribution,
    /// Information about the max fee bid.
    pub max_fee: FeeDistribution,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LedgerLinks {
    /// Link to this ledger.
    #[serde(rename = "self")]
    pub self_: Link,
    /// Link to the transactions in the ledger.
    pub transactions: Link,
    /// Link to the operations in the ledger.
    pub operations: Link,
    /// Link to the payments in the ledger.
    pub payments: Link,
    /// Link to the effects in the ledger.
    pub effects: Link,
}
