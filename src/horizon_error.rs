//! Horizon error response.
use serde::{Deserialize, Serialize};

const BAD_REQUEST_TYPE: &str = "https://stellar.org/horizon-errors/bad_request";
const TRANSACTION_FAILED_TYPE: &str = "https://stellar.org/horizon-errors/transaction_failed";
const TRANSACTION_MALFORMED_TYPE: &str = "https://stellar.org/horizon-errors/transaction_malformed";
const BEFORE_HISTORY_TYPE: &str = "https://stellar.org/horizon-errors/before_history";
const STALE_HISTORY_TYPE: &str = "https://stellar.org/horizon-errors/stale_history";
const TIMEOUT_TYPE: &str = "https://stellar.org/horizon-errors/timeout";

/// Horizon error response.
#[derive(Debug, Clone, PartialEq)]
pub enum HorizonError {
    BadRequest(HorizonErrorBadRequest),
    TransactionFailed(HorizonErrorTransactionFailed),
    TransactionMalformed(HorizonErrorTransactionMalformed),
    BeforeHistory(HorizonErrorBase),
    StaleHistory(HorizonErrorBase),
    Timeout(HorizonErrorBase),
    Other(HorizonErrorBase),
}

/// Common fields in horizon error responses.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorBase {
    #[serde(rename = "type")]
    pub url: String,
    /// A short description of the error.
    pub title: String,
    /// A longer description of the error.
    pub detail: String,
    /// The status code.
    pub status: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorBadRequest {
    #[serde(flatten)]
    pub base: HorizonErrorBase,
    pub extras: HorizonErrorBadRequestExtras,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorBadRequestExtras {
    pub invalid_field: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorTransactionFailed {
    #[serde(flatten)]
    pub base: HorizonErrorBase,
    pub extras: HorizonErrorTransactionFailedExtras,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorTransactionFailedExtras {
    pub envelope_xdr: String,
    pub result_xdr: String,
    pub result_codes: HorizonErrorTransactionFailedResultCodes,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorTransactionFailedResultCodes {
    pub transaction: String,
    #[serde(default)]
    pub operations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorTransactionMalformed {
    #[serde(flatten)]
    pub base: HorizonErrorBase,
    pub extras: HorizonErrorTransactionMalformedExtras,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HorizonErrorTransactionMalformedExtras {
    pub envelope_xdr: String,
}

impl<'de> serde::Deserialize<'de> for HorizonError {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json_value: serde_json::Value = serde::Deserialize::deserialize(de)?;
        let error_object = json_value
            .as_object()
            .ok_or(serde::de::Error::custom("expected a stellar error object"))?;
        let error_type = error_object
            .get("type")
            .ok_or(serde::de::Error::custom("expected type field"))?;
        let error_type_str = error_type.as_str().ok_or(serde::de::Error::custom(
            "expected type field to be a string",
        ))?;

        match error_type_str {
            BAD_REQUEST_TYPE => {
                let horizon_error: HorizonErrorBadRequest = serde_json::from_value(json_value)
                    .map_err(|_| serde::de::Error::custom("bad_request"))?;
                Ok(HorizonError::BadRequest(horizon_error))
            }
            TRANSACTION_FAILED_TYPE => {
                let horizon_error: HorizonErrorTransactionFailed =
                    serde_json::from_value(json_value)
                        .map_err(|_| serde::de::Error::custom("transaction_failed"))?;
                Ok(HorizonError::TransactionFailed(horizon_error))
            }
            TRANSACTION_MALFORMED_TYPE => {
                let horizon_error: HorizonErrorTransactionMalformed =
                    serde_json::from_value(json_value)
                        .map_err(|_| serde::de::Error::custom("transaction_malformed"))?;
                Ok(HorizonError::TransactionMalformed(horizon_error))
            }
            BEFORE_HISTORY_TYPE => {
                let horizon_error: HorizonErrorBase = serde_json::from_value(json_value)
                    .map_err(|_| serde::de::Error::custom("before_history"))?;
                Ok(HorizonError::BeforeHistory(horizon_error))
            }
            STALE_HISTORY_TYPE => {
                let horizon_error: HorizonErrorBase = serde_json::from_value(json_value)
                    .map_err(|_| serde::de::Error::custom("stale_history"))?;
                Ok(HorizonError::StaleHistory(horizon_error))
            }
            TIMEOUT_TYPE => {
                let horizon_error: HorizonErrorBase = serde_json::from_value(json_value)
                    .map_err(|_| serde::de::Error::custom("timeout"))?;
                Ok(HorizonError::Timeout(horizon_error))
            }
            _ => {
                let horizon_error: HorizonErrorBase = serde_json::from_value(json_value)
                    .map_err(|_| serde::de::Error::custom("bad_request"))?;
                Ok(HorizonError::Other(horizon_error))
            }
        }
    }
}

impl serde::Serialize for HorizonError {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HorizonError::BadRequest(horizon_error) => horizon_error.serialize(ser),
            HorizonError::TransactionFailed(horizon_error) => horizon_error.serialize(ser),
            HorizonError::TransactionMalformed(horizon_error) => horizon_error.serialize(ser),
            HorizonError::BeforeHistory(horizon_error) => horizon_error.serialize(ser),
            HorizonError::StaleHistory(horizon_error) => horizon_error.serialize(ser),
            HorizonError::Timeout(horizon_error) => horizon_error.serialize(ser),
            HorizonError::Other(horizon_error) => horizon_error.serialize(ser),
        }
    }
}
