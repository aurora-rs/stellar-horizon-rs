use crate::api::Join;
use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources::{self, LedgerId};
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

/// Creates a request to retrieve a ledger's payments.
pub fn for_ledger(ledger_id: LedgerId) -> PaymentsForLedgerRequest {
    PaymentsForLedgerRequest {
        ledger_id,
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a transaction's payments.
pub fn for_transaction<S>(tx_hash: S) -> PaymentsForTransactionRequest
where
    S: Into<String>,
{
    PaymentsForTransactionRequest {
        tx_hash: tx_hash.into(),
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

impl AllPaymentsRequest {
    impl_include_failed!();
    impl_join!();
}

impl PaymentsForAccountRequest {
    impl_include_failed!();
    impl_join!();
}

impl PaymentsForLedgerRequest {
    impl_include_failed!();
    impl_join!();
}

impl PaymentsForTransactionRequest {
    impl_include_failed!();
    impl_join!();
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

/// Request payments for a ledger.
#[derive(Debug, Clone)]
pub struct PaymentsForLedgerRequest {
    ledger_id: LedgerId,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request payments for a transaction.
#[derive(Debug, Clone)]
pub struct PaymentsForTransactionRequest {
    tx_hash: String,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllPaymentsRequest {
    type Response = Page<resources::Payment>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("payments")?;
        url = url.append_include_failed(&self.include_failed);
        url = url.appen_join(&self.join);
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
        let mut url = host.join(&format!("accounts/{}/payments", self.account_id))?;
        url = url.append_include_failed(&self.include_failed);
        url = url.appen_join(&self.join);
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(PaymentsForAccountRequest);

impl StreamRequest for PaymentsForAccountRequest {
    type Resource = resources::Payment;
}

impl Request for PaymentsForLedgerRequest {
    type Response = Page<resources::Payment>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("ledgers/{}/payments", self.ledger_id))?;
        url = url.append_include_failed(&self.include_failed);
        url = url.appen_join(&self.join);
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(PaymentsForLedgerRequest);

impl Request for PaymentsForTransactionRequest {
    type Response = Page<resources::Payment>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join(&format!("transactions/{}/payments", self.tx_hash))?;
        url = url.append_include_failed(&self.include_failed);
        url = url.appen_join(&self.join);
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(PaymentsForTransactionRequest);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Join;
    use crate::request::Request;
    use std::collections::HashMap;
    use stellar_base::crypto::PublicKey;
    use url::Url;

    fn host() -> Url {
        "https://horizon.stellar.org".parse().unwrap()
    }

    #[test]
    fn test_all_payments_request_uri() {
        let req = all()
            .with_include_failed(true)
            .with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/payments?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"true".to_string()), query.get("include_failed"));
    }

    #[test]
    fn test_payments_for_account_request_uri() {
        let pk =
            PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
                .unwrap();
        let req = for_account(&pk)
            .with_cursor("now")
            .with_join(Join::Transactions)
            .with_include_failed(true);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/payments?"));
    }

    #[test]
    fn test_payments_for_ledger_request_uri() {
        let req = for_ledger(888)
            .with_include_failed(true)
            .with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers/888/payments?"));
    }

    #[test]
    fn test_payments_for_transaction_request_uri() {
        let req =
            for_transaction("715ffb63673a4ee9b84d4b60924b3e141b34fe3777697f35bad6d4b990524ca2")
                .with_include_failed(true)
                .with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/transactions/715ffb63673a4ee9b84d4b60924b3e141b34fe3777697f35bad6d4b990524ca2/payments?"));
    }
}
