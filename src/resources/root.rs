use crate::resources::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "_links")]
    pub links: RootLinks,
    pub horizon_version: String,
    pub core_version: String,
    pub ingest_latest_ledger: u32,
    pub history_latest_ledger: i32,
    pub history_elder_ledger: i32,
    pub core_latest_ledger: i32,
    pub network_passphrase: String,
    pub current_protocol_version: i32,
    pub core_supported_protocol_version: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootLinks {
    pub account: Link,
    pub accounts: Option<Link>,
    pub account_transactions: Link,
    pub assets: Link,
    pub effects: Link,
    pub fee_stats: Link,
    pub friendbot: Option<Link>,
    pub ledger: Link,
    pub ledgers: Link,
    pub offer: Option<Link>,
    pub offers: Option<Link>,
    pub operation: Link,
    pub operations: Link,
    pub order_book: Link,
    pub payments: Link,
    #[serde(rename = "self")]
    pub self_: Link,
    pub strict_receive_paths: Option<Link>,
    pub strict_send_paths: Option<Link>,
    pub trade_aggregations: Link,
    pub trades: Link,
    pub transaction: Link,
    pub transactions: Link,
}
