use crate::api::{Join, Page};
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::PublicKey;
use url::Url;

/// Creates a request to retrieve all payments.
pub fn all() -> AllPaymentsRequest {
    Default::default()
}

/// Creates a request to retrieve the account's payments.
pub fn for_account(account: &PublicKey) -> PaymentsForAccountRequest {
    PaymentsForAccountRequest {
        account_id: account.account_id(),
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

impl AllPaymentsRequest {
    pub fn with_include_failed(mut self, include_failed: bool) -> Self {
        self.include_failed = Some(include_failed);
        self
    }

    pub fn with_join(mut self, join: Join) -> Self {
        self.join = Some(join);
        self
    }
}

impl PaymentsForAccountRequest {
    pub fn with_include_failed(mut self, include_failed: bool) -> Self {
        self.include_failed = Some(include_failed);
        self
    }

    pub fn with_join(mut self, join: Join) -> Self {
        self.join = Some(join);
        self
    }
}

/// Request all payments.
#[derive(Debug, Clone, Default)]
pub struct AllPaymentsRequest {
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request payments for an account.
#[derive(Debug, Clone)]
pub struct PaymentsForAccountRequest {
    account_id: String,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllPaymentsRequest {
    type Response = Page<resources::Payment>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/payments")?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        if let Some(join) = self.join {
            url = url.append_query_param("join", &join.to_query_value());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllPaymentsRequest);

impl StreamRequest for AllPaymentsRequest {
    type Resource = resources::Payment;
}

impl Request for PaymentsForAccountRequest {
    type Response = Page<resources::Payment>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("/accounts/{}/payments", self.account_id))?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        if let Some(join) = self.join {
            url = url.append_query_param("join", &join.to_query_value());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(PaymentsForAccountRequest);

impl StreamRequest for PaymentsForAccountRequest {
    type Resource = resources::Payment;
}
