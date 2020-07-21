use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::PublicKey;
use url::Url;

/// Create a request to retrieve all effects.
pub fn all() -> AllEffectsRequest {
    Default::default()
}

/// Create a request to retrieve effects for a transaction.
pub fn for_transaction(tx_hash: String) -> EffectsForTransactionRequest {
    EffectsForTransactionRequest {
        tx_hash,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Create a request to retrieve effects for an operation.
pub fn for_operation(operation_id: String) -> EffectsForOperationRequest {
    EffectsForOperationRequest {
        operation_id,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Create a request to retrieve effects for a ledger.
pub fn for_ledger(ledger: i32) -> EffectsForLedgerRequest {
    EffectsForLedgerRequest {
        ledger,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Create a request to retrieve effects for an account.
pub fn for_account(account: &PublicKey) -> EffectsForAccountRequest {
    EffectsForAccountRequest {
        account_id: account.account_id(),
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Request all effects.
#[derive(Debug, Clone, Default)]
pub struct AllEffectsRequest {
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request effects for a ledger.
#[derive(Debug, Clone)]
pub struct EffectsForLedgerRequest {
    ledger: i32,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request effects for a transaction.
#[derive(Debug, Clone)]
pub struct EffectsForTransactionRequest {
    tx_hash: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request effects for an operation.
#[derive(Debug, Clone)]
pub struct EffectsForOperationRequest {
    operation_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request effects for an account.
#[derive(Debug, Clone)]
pub struct EffectsForAccountRequest {
    account_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllEffectsRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join("/effects")?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllEffectsRequest);

impl StreamRequest for AllEffectsRequest {
    type Resource = resources::Effect;
}

impl Request for EffectsForLedgerRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/ledgers/{}/effects", self.ledger))?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForLedgerRequest);

impl StreamRequest for EffectsForLedgerRequest {
    type Resource = resources::Effect;
}

impl Request for EffectsForTransactionRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/transactions/{}/effects", self.tx_hash))?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForTransactionRequest);

impl Request for EffectsForOperationRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/operations/{}/effects", self.operation_id))?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForOperationRequest);

impl Request for EffectsForAccountRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/accounts/{}/effects", self.account_id))?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForAccountRequest);

impl StreamRequest for EffectsForAccountRequest {
    type Resource = resources::Effect;
}
