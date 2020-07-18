use crate::resources::{Asset, Link};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "_links")]
    pub links: AccountLinks,
    pub id: String,
    pub account_id: String,
    pub sequence: String,
    pub subentry_count: i32,
    pub inflation_destination: Option<String>,
    pub home_domain: Option<String>,
    pub last_modified_ledger: u32,
    pub last_modified_time: Option<DateTime<Utc>>,
    pub thresholds: AccountThresholds,
    pub flags: AccountFlags,
    pub balances: Vec<Balance>,
    pub signers: Vec<Signer>,
    pub data: Map<String, String>,
    pub paging_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub transactions: Link,
    pub operations: Link,
    pub payments: Link,
    pub effects: Link,
    pub offers: Link,
    pub trades: Link,
    pub data: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountThresholds {
    pub low_threshold: u8,
    #[serde(rename = "med_threshold")]
    pub medium_threshold: u8,
    pub high_threshold: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountFlags {
    pub auth_required: bool,
    pub auth_revocable: bool,
    pub auth_immutable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Balance {
    pub balance: String,
    pub limit: Option<String>,
    pub buying_liabilities: String,
    pub selling_liabilities: String,
    pub last_modified_ledger: Option<u32>,
    pub is_authorized: Option<bool>,
    pub is_authorized_to_maintain_liabilities: Option<bool>,
    #[serde(flatten)]
    pub asset: Asset,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountSigner {
    #[serde(rename = "_links")]
    pub links: AccountSignerLinks,
    pub id: String,
    pub account_id: String,
    pub paging_token: String,
    pub signer: Signer,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountData {
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountSignerLinks {
    pub account: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signer {
    pub weight: i32,
    pub key: String,
    #[serde(rename = "type")]
    pub type_: String,
}
