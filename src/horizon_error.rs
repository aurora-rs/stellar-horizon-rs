use serde::{Deserialize, Serialize};

/// Horizon error response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HorizonError {
    /// A short description of the error.
    pub title: String,
}
