use crate::api::Join;
use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::PublicKey;
use url::Url;

/// Creates a request to retrieve all operations.
pub fn all() -> AllOperationsRequest {
    Default::default()
}

/// Creates a request to retrieve a single operation.
pub fn single(operation_id: String) -> SingleOperationRequest {
    SingleOperationRequest {
        operation_id,
        join: None,
    }
}

/// Creates a request to retrieve the account's operations.
pub fn for_account(account: &PublicKey) -> OperationsForAccountRequest {
    OperationsForAccountRequest {
        account_id: account.account_id(),
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a ledger's operations.
pub fn for_ledger(ledger: i32) -> OperationsForLedgerRequest {
    OperationsForLedgerRequest {
        ledger,
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

impl AllOperationsRequest {
    pub fn with_include_failed(mut self, include_failed: bool) -> Self {
        self.include_failed = Some(include_failed);
        self
    }

    pub fn with_join(mut self, join: Join) -> Self {
        self.join = Some(join);
        self
    }
}

impl SingleOperationRequest {
    pub fn with_join(mut self, join: Join) -> Self {
        self.join = Some(join);
        self
    }
}

impl OperationsForAccountRequest {
    pub fn with_include_failed(mut self, include_failed: bool) -> Self {
        self.include_failed = Some(include_failed);
        self
    }

    pub fn with_join(mut self, join: Join) -> Self {
        self.join = Some(join);
        self
    }
}

impl OperationsForLedgerRequest {
    pub fn with_include_failed(mut self, include_failed: bool) -> Self {
        self.include_failed = Some(include_failed);
        self
    }

    pub fn with_join(mut self, join: Join) -> Self {
        self.join = Some(join);
        self
    }
}

/// Request all operations.
#[derive(Debug, Clone, Default)]
pub struct AllOperationsRequest {
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request a single operation.
#[derive(Debug, Clone)]
pub struct SingleOperationRequest {
    operation_id: String,
    join: Option<Join>,
}

/// Request an account operations.
#[derive(Debug, Clone)]
pub struct OperationsForAccountRequest {
    account_id: String,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request a ledger operations.
#[derive(Debug, Clone)]
pub struct OperationsForLedgerRequest {
    ledger: i32,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllOperationsRequest {
    type Response = Page<resources::Operation>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/operations")?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        if let Some(join) = self.join {
            url = url.append_query_param("join", &join.to_query_value());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllOperationsRequest);

impl StreamRequest for AllOperationsRequest {
    type Resource = resources::Operation;
}

impl Request for SingleOperationRequest {
    type Response = resources::Operation;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("/operations/{}", self.operation_id))?;
        if let Some(join) = self.join {
            url = url.append_query_param("join", &join.to_query_value());
        }
        Ok(url)
    }
}

impl Request for OperationsForAccountRequest {
    type Response = Page<resources::Operation>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("/accounts/{}/operations", self.account_id))?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        if let Some(join) = self.join {
            url = url.append_query_param("join", &join.to_query_value());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(OperationsForAccountRequest);

impl StreamRequest for OperationsForAccountRequest {
    type Resource = resources::Operation;
}

impl Request for OperationsForLedgerRequest {
    type Response = Page<resources::Operation>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("/ledgers/{}/operations", self.ledger))?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        if let Some(join) = self.join {
            url = url.append_query_param("join", &join.to_query_value());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(OperationsForLedgerRequest);

impl StreamRequest for OperationsForLedgerRequest {
    type Resource = resources::Operation;
}
