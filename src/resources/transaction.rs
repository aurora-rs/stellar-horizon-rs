use crate::resources::{Asset, Link, Price};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(rename = "_links")]
    pub links: TransactionLinks,
    pub id: String,
    pub paging_token: String,
    pub successful: bool,
    pub hash: String,
    pub ledger: i32,
    pub created_at: DateTime<Utc>,
    pub source_account: String,
    pub source_account_sequence: String,
    pub fee_account: String,
    pub fee_charged: String,
    pub max_fee: String,
    pub operation_count: i32,
    pub envelope_xdr: String,
    pub result_xdr: String,
    pub result_meta_xdr: String,
    pub fee_meta_xdr: String,
    pub memo_type: String,
    pub memo_bytes: Option<String>,
    pub memo: Option<String>,
    pub signatures: Vec<String>,
    pub valid_after: Option<String>,
    pub valid_before: Option<String>,
    pub fee_bump_transaction: Option<FeeBumpTransaction>,
    pub inner_transaction: Option<InnerTransaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeBumpTransaction {
    pub hash: String,
    pub signatures: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InnerTransaction {
    pub hash: String,
    pub signatures: Vec<String>,
    pub max_fee: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionResultCodes {
    pub transaction: String,
    pub operations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub account: Link,
    pub ledger: Link,
    pub operations: Link,
    pub effects: Link,
    pub precedes: Link,
    pub succeeds: Link,
    pub transaction: Link,
}
