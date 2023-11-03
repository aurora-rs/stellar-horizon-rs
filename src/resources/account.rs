use crate::link::Link;
use crate::resources::Asset;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

/// User accounts on the network.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Account {
    /// Account links.
    #[serde(rename = "_links")]
    pub links: AccountLinks,
    /// An unique identifier for this account.
    pub id: String,
    /// This account's public key encoded as base32 string.
    pub account_id: String,
    /// This account's current sequence number.
    pub sequence: String,
    /// The number of subentries in this account.
    pub subentry_count: i32,
    /// The inflation destination.
    pub inflation_destination: Option<String>,
    /// The domain that host this account's `stellar.toml` file.
    pub home_domain: Option<String>,
    /// The id of the last ledger that included changes to this account.
    pub last_modified_ledger: u32,
    /// The time when this account was last modified.
    pub last_modified_time: Option<DateTime<Utc>>,
    /// Thresholds for different access levels.
    pub thresholds: AccountThresholds,
    /// Flags for enablding/disabling of certain asset issuer privileges.
    pub flags: AccountFlags,
    /// The assets this account holds.
    pub balances: Vec<Balance>,
    /// The signers with weights that can be used to sign transactions for this account.
    pub signers: Vec<Signer>,
    /// Account data.
    pub data: Map<String, String>,
    /// The number of reserves sponsored by this account.
    #[serde(default)]
    pub num_sponsoring: i64,
    /// The number of reserves sponsored for this account.
    #[serde(default)]
    pub num_sponsored: i64,
    /// The account sponsoring this account base reserve.
    pub sponsor: Option<String>,
    /// Paging token for this account.
    pub paging_token: String,
}

/// Links for an Account.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountLinks {
    /// This account.
    #[serde(rename = "self")]
    pub self_: Link,
    /// This account's transactions.
    pub transactions: Link,
    /// This account's operation.
    pub operations: Link,
    /// This account's payments.
    pub payments: Link,
    /// This account's effects.
    pub effects: Link,
    /// This account's offers.
    pub offers: Link,
    /// This account's trades.
    pub trades: Link,
    /// This account's data.
    pub data: Link,
}

/// Thresholds for different access levels.
///
///  * Low Threshold: allow trust, bump sequence
///  * Medium Threshold: create account, payment, path payment, manage
///      buy offer, manage sell offer, create passive sell offer, change
///      trust, inflation, manage data
///  * High Threshold: account merge, set options
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountThresholds {
    /// The weight for a low threshold transaction.
    pub low_threshold: u8,
    /// The weight for a medium threshold transaction.
    #[serde(rename = "med_threshold")]
    pub medium_threshold: u8,
    /// The weight for a high threshold transaction.
    pub high_threshold: u8,
}

/// Enabling/disabling of certain asset issuet privilege.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountFlags {
    /// If `true`, none of the following flags can be changed.
    pub auth_required: bool,
    /// If `true`, anyone who wants to hold an asset issued by this
    /// account must first be approved by this account.
    pub auth_revocable: bool,
    /// If `true`, this account can freeze the balance of a holder of an asset issued by this account.
    pub auth_immutable: bool,
    /// If `true`, trustlines created for assets issued by this account have clawbacks enabled.
    pub auth_clawback_enabled: bool,
}

/// Asset balance.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Balance {
    /// The number of units the account holds.
    pub balance: String,
    pub liquidity_pool_id: Option<String>,
    /// The maximum amount of the asset the account is willing to accept.
    pub limit: Option<String>,
    /// The sum of all buy offers owned by this account for this asset.
    pub buying_liabilities: Option<String>,
    /// The sum of all sell offers owned by this account for this asset.
    pub selling_liabilities: Option<String>,
    /// The account sponsoring this trustline.
    pub sponsor: Option<String>,
    /// Ledger when the balance was last changed.
    pub last_modified_ledger: Option<u32>,
    /// Flag to indicate if the account is authorized to hold asset.
    pub is_authorized: Option<bool>,
    /// Flag to indicate if the account is authorized to maintain liabilities.
    pub is_authorized_to_maintain_liabilities: Option<bool>,
    pub is_clawback_enabled: Option<bool>,
    /// The asset.
    #[serde(flatten)]
    pub asset: Asset,
}

/// A valid signer for the account.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountSigner {
    /// Account signer links.
    #[serde(rename = "_links")]
    pub links: AccountSignerLinks,
    /// Signer unique identifier.
    pub id: String,
    /// Signer account id.
    pub account_id: String,
    /// Signer paging token.
    pub paging_token: String,
    /// The signer.
    pub signer: Signer,
}

/// Account data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountData {
    /// Account data encoded as base64.
    pub value: String,
    /// Account sponsoring this data base reserve.
    pub sponsor: Option<String>,
}

/// Account signer links.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountSignerLinks {
    /// Link to the account.
    pub account: Link,
}

/// Account signer
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Signer {
    /// Signer weight.
    pub weight: i32,
    /// Signer key, depends on the signer type.
    pub key: String,
    /// The signer type.
    #[serde(rename = "type")]
    pub type_: String,
    /// The account sponsoring this signer base reserve.
    pub sponsor: Option<String>,
}
