//! Horizon API requests builders and types.
pub mod accounts;
pub mod aggregations;
pub mod assets;
pub mod claimable_balances;
pub mod data;
pub mod effects;
pub mod ledgers;
pub mod offers;
pub mod operations;
pub mod payments;
pub mod root;
pub mod trades;
pub mod transactions;

/// Optionally join data with the operations response.
#[derive(Debug, Copy, Clone)]
pub enum Join {
    /// Include the operation transaction.
    Transactions,
}

impl Join {
    /// Return the order query value.
    pub fn to_query_value(&self) -> String {
        match self {
            Join::Transactions => "transactions".to_string(),
        }
    }
}
