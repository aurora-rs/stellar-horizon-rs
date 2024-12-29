use crate::link::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Transactions are commands that modify the ledger state and consist of one or more operations.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    /// Transaction links.
    #[serde(rename = "_links")]
    pub links: TransactionLinks,
    /// An unique identifier for this transaction.
    pub id: String,
    /// A cursor value for use in pagination.
    pub paging_token: String,
    /// Indicates if this transaction was successful or not.
    pub successful: bool,
    /// A hex-encoded SHA-256 hash of this transaction’s XDR-encoded form.
    pub hash: String,
    /// The sequence number of the ledger that this transaction was included in.
    pub ledger: i32,
    /// The date this transaction was created.
    pub created_at: DateTime<Utc>,
    /// The account that originates the transaction.
    pub source_account: String,
    /// The muxed account that was the source account.
    pub account_muxed: Option<String>,
    /// The ID of the muxed account that was the source account.
    pub account_muxed_id: Option<String>,
    /// The source account’s sequence number that this transaction consumed.
    pub source_account_sequence: String,
    /// The account that paid this transaction fee.
    pub fee_account: String,
    /// The muxed account that paid this transaction fee.
    pub fee_account_muxed: Option<String>,
    /// The ID of the muxed account that paid this transaction fee.
    pub fee_account_muxed_id: Option<String>,
    /// The fee (in stroops) paid by the source account to apply this transaction to the ledger.
    #[serde_as(as = "DisplayFromStr")]
    pub fee_charged: i64,
    /// The maximum fee (in stroops) that the source account was willing to pay.
    #[serde_as(as = "DisplayFromStr")]
    pub max_fee: i64,
    /// The number of operations contained within this transaction.
    pub operation_count: i32,
    /// A base64 encoded string of the raw `TransactionEnvelope` XDR struct for this transaction.
    pub envelope_xdr: String,
    /// A base64 encoded string of the raw `TransactionResult` XDR struct for this transaction.
    pub result_xdr: String,
    /// A base64 encoded string of the raw `TransactionMeta` XDR struct for this transaction.
    pub result_meta_xdr: Option<String>,
    /// A base64 encoded string of the raw `LedgerEntryChanges` XDR struct produced by taking fees for this transaction.
    pub fee_meta_xdr: String,
    /// The type of memo. Potential values include `MEMO_TEXT`, `MEMO_ID`, `MEMO_HASH`, `MEMO_RETURN`.
    pub memo_type: String,
    /// The optional memo attached to a transaction, in base64 encoded bytes.
    pub memo_bytes: Option<String>,
    /// The optional memo attached to a transaction.
    pub memo: Option<String>,
    /// An array of signatures used to sign this transaction.
    pub signatures: Vec<String>,
    /// The date after which a transaction is valid.
    pub valid_after: Option<String>,
    /// The date before which a transaction is valid.
    pub valid_before: Option<String>,
    /// The fee bump transaction.
    pub fee_bump_transaction: Option<FeeBumpTransaction>,
    /// The fee bump inner transaction.
    pub inner_transaction: Option<InnerTransaction>,
}

/// Fee bump transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FeeBumpTransaction {
    /// The transaction hash.
    pub hash: String,
    /// An array of signatures used to sign this transaction.
    pub signatures: Vec<String>,
}

/// Fee bump transaction inner transaction.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InnerTransaction {
    /// The transaction hash.
    pub hash: String,
    /// An array of signatures used to sign this transaction.
    pub signatures: Vec<String>,
    /// The transaction max fee.
    #[serde_as(as = "DisplayFromStr")]
    pub max_fee: i64,
}

/// Transaction result codes.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionResultCodes {
    /// The transaction.
    pub transaction: String,
    /// The operations.
    pub operations: Vec<String>,
}

/// Transaction links.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionLinks {
    /// This transaction.
    #[serde(rename = "self")]
    pub self_: Link,
    /// Link to source account.
    pub account: Link,
    /// Link to the ledger.
    pub ledger: Link,
    /// Link to operations in the transaction.
    pub operations: Link,
    /// Link to effects.
    pub effects: Link,
    /// Link to the transaction before this one.
    pub precedes: Link,
    /// Link to the transaction after this one.
    pub succeeds: Link,
    /// Link to the transaction.
    pub transaction: Link,
}
