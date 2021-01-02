//! Horizon error response.
use serde::{Deserialize, Serialize};

/// Result codes for individual operations related to horizon error response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HorizonErrorExtrasResultCodes {
    /// The transaction Result Code returned by Stellar Core, which can be used to look up more information about an error in the docs.
    pub transaction: String,
    /// An array of operation Result Codes returned by Stellar Core, which can be used to look up more information about an error in the docs.
    pub operations: Vec<String>,
}

/// Extra information related to horizon error response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HorizonErrorExtras {
    /// A base64-encoded representation of the TransactionEnvelope XDR whose failure triggered this response.
    pub envelope_xdr: String,
    /// A base64-encoded representation of the TransactionResult XDR returned by stellar-core when submitting this transaction.
    pub result_xdr: String,
    /// Result codes for the individual operations
    pub result_codes: HorizonErrorExtrasResultCodes,
}

/// Horizon error response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HorizonError {
    /// A short description of the error.
    pub title: String,
    /// A longer description of the error.
    pub detail: String,
    /// The status code.
    pub status: i64,
    /// If the Status Code is Transaction Failed, this extras field displays the Result Code returned by Stellar Core describing why the transaction failed.
    pub extras: Option<HorizonErrorExtras>,
}
