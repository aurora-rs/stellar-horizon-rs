use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::crypto::PublicKey;
use stellar_base::transaction::TransactionEnvelope;
use stellar_base::xdr::XDRSerialize;
use url::{form_urlencoded, Url};

/// Creates a request to retrieve all transactions.
pub fn all() -> AllTransactionsRequest {
    AllTransactionsRequest {
        include_failed: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a single transaction.
pub fn single<S: Into<String>>(id: S) -> SingleTransactionRequest {
    SingleTransactionRequest { id: id.into() }
}

/// Creates a request to submit a transaction.
pub fn submit(tx: &TransactionEnvelope) -> Result<SubmitTransactionRequest> {
    let xdr = tx.xdr_base64()?;
    Ok(SubmitTransactionRequest { xdr })
}

/// Creates a request to retrieve a account's transactions.
pub fn for_account(account: &PublicKey) -> TransactionsForAccountRequest {
    TransactionsForAccountRequest {
        account_id: account.account_id(),
        include_failed: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a ledger's request.
pub fn for_ledger(ledger: u32) -> TransactionsForLedgerRequest {
    TransactionsForLedgerRequest {
        ledger,
        include_failed: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Request all transactions.
#[derive(Debug, Clone)]
pub struct AllTransactionsRequest {
    include_failed: Option<bool>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request a single transaction.
#[derive(Debug, Clone)]
pub struct SingleTransactionRequest {
    id: String,
}

/// Submit a transaction.
#[derive(Debug, Clone)]
pub struct SubmitTransactionRequest {
    xdr: String,
}

/// Request an account's transaction.
#[derive(Debug, Clone)]
pub struct TransactionsForAccountRequest {
    include_failed: Option<bool>,
    account_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request a ledger's transaction.
#[derive(Debug, Clone)]
pub struct TransactionsForLedgerRequest {
    include_failed: Option<bool>,
    ledger: u32,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl AllTransactionsRequest {
    impl_include_failed!();
}

impl Request for AllTransactionsRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/transactions")?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllTransactionsRequest);

impl StreamRequest for AllTransactionsRequest {
    type Resource = resources::Transaction;
}

impl Request for SingleTransactionRequest {
    type Response = resources::Transaction;

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join(&format!("/transactions/{}", self.id))?)
    }
}

impl Request for SubmitTransactionRequest {
    type Response = resources::Transaction;

    fn post_body(&self) -> Result<Option<String>> {
        let body = form_urlencoded::Serializer::new(String::new())
            .append_pair("tx", &self.xdr)
            .finish();
        Ok(Some(body))
    }

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join("/transactions")?)
    }
}

impl TransactionsForAccountRequest {
    impl_include_failed!();
}

impl Request for TransactionsForAccountRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("/accounts/{}/transactions", self.account_id))?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForAccountRequest);

impl TransactionsForLedgerRequest {
    impl_include_failed!();
}

impl Request for TransactionsForLedgerRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("/ledgers/{}/transactions", self.ledger))?;
        if let Some(include_failed) = self.include_failed {
            url = url.append_query_param("include_failed", &include_failed.to_string());
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForLedgerRequest);
