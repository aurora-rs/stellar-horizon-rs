use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources::{self, ClaimableBalanceId, LedgerId};
use stellar_base::crypto::PublicKey;
use stellar_base::transaction::TransactionEnvelope;
use stellar_base::xdr::XDRSerialize;
use url::{form_urlencoded, Url};

use super::{accounts, claimable_balances, ledgers, liquidity_pools};

pub(crate) const API_PATH: &str = "transactions";

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

/// Creates a request to retrieve a ledger's transactions.
pub fn for_ledger(ledger: LedgerId) -> TransactionsForLedgerRequest {
    TransactionsForLedgerRequest {
        ledger,
        include_failed: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

pub fn for_claimable_balance<S: Into<String>>(
    claimable_balance_id: S,
) -> TransactionsForClaimableBalanceRequest {
    TransactionsForClaimableBalanceRequest {
        claimable_balance_id: claimable_balance_id.into(),
        include_failed: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a transactions linked to liquidity pool.
pub fn for_liquidity_pool<S: Into<String>>(
    liquidity_pool_id: S,
) -> TransactionsForLiquidityPoolRequest {
    TransactionsForLiquidityPoolRequest {
        liquidity_pool_id: liquidity_pool_id.into(),
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
    ledger: LedgerId,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request transactions linked to a claimable balance.
#[derive(Debug, Clone)]
pub struct TransactionsForClaimableBalanceRequest {
    include_failed: Option<bool>,
    claimable_balance_id: ClaimableBalanceId,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request transaction linked to a liquidity pool.
#[derive(Debug, Clone)]
pub struct TransactionsForLiquidityPoolRequest {
    liquidity_pool_id: String,
    include_failed: Option<bool>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl AllTransactionsRequest {
    impl_include_failed!();
}

impl Request for AllTransactionsRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllTransactionsRequest);

impl StreamRequest for AllTransactionsRequest {
    type Resource = resources::Transaction;
}

impl Request for SingleTransactionRequest {
    type Response = resources::Transaction;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH, self.id.as_str()]);
        }
        Ok(base_url)
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

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        Ok(base_url)
    }
}

impl TransactionsForAccountRequest {
    impl_include_failed!();
}

impl Request for TransactionsForAccountRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[accounts::API_PATH, self.account_id.as_str(), API_PATH]);
        }
        Ok(base_url
            .append_include_failed(&self.include_failed)
            .append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForAccountRequest);

impl StreamRequest for TransactionsForAccountRequest {
    type Resource = resources::Transaction;
}

impl TransactionsForLedgerRequest {
    impl_include_failed!();
}

impl Request for TransactionsForLedgerRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            let ledger = self.ledger.to_string();
            segments.extend(&[ledgers::API_PATH, ledger.as_str(), API_PATH]);
        }
        Ok(base_url
            .append_include_failed(&self.include_failed)
            .append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForLedgerRequest);

impl StreamRequest for TransactionsForLedgerRequest {
    type Resource = resources::Transaction;
}

impl TransactionsForClaimableBalanceRequest {
    impl_include_failed!();
}

impl Request for TransactionsForClaimableBalanceRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[
                claimable_balances::API_PATH,
                self.claimable_balance_id.as_str(),
                API_PATH,
            ]);
        }
        Ok(base_url
            .append_include_failed(&self.include_failed)
            .append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForClaimableBalanceRequest);

impl StreamRequest for TransactionsForClaimableBalanceRequest {
    type Resource = resources::Transaction;
}

impl TransactionsForLiquidityPoolRequest {
    impl_include_failed!();
}

impl Request for TransactionsForLiquidityPoolRequest {
    type Response = Page<resources::Transaction>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[
                liquidity_pools::API_PATH,
                self.liquidity_pool_id.as_str(),
                API_PATH,
            ]);
        }

        Ok(base_url
            .append_include_failed(&self.include_failed)
            .append_pagination_params(self))
    }
}

impl_page_request!(TransactionsForLiquidityPoolRequest);

impl StreamRequest for TransactionsForLiquidityPoolRequest {
    type Resource = resources::Transaction;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Request;
    use stellar_base::crypto::PublicKey;
    use url::Url;

    fn host() -> Url {
        "https://horizon.stellar.org".parse().unwrap()
    }

    fn base_url() -> Url {
        "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap()
    }

    #[test]
    fn test_all_transactions_request_uri() {
        let req = all().with_include_failed(true);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/transactions?"));
    }

    #[test]
    fn test_all_transactions_request_uri_with_base_url() {
        let req = all().with_include_failed(true);
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/transactions?"));
    }

    #[test]
    fn test_single_transaction_request_uri() {
        let req = single("23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e");
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/transactions/23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e"));
    }

    #[test]
    fn test_single_transaction_request_uri_with_base_url() {
        let req = single("23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e");
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/transactions/23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e"));
    }

    #[test]
    fn test_transactions_for_account_request_uri() {
        let pk =
            PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
                .unwrap();
        let req = for_account(&pk).with_include_failed(true);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/transactions?"));
    }

    #[test]
    fn test_transactions_for_account_request_uri_with_base_url() {
        let pk =
            PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
                .unwrap();
        let req = for_account(&pk).with_include_failed(true);
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/transactions?"));
    }

    #[test]
    fn test_transactions_for_ledger_request_uri() {
        let req = for_ledger(888).with_include_failed(true);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers/888/transactions?"));
    }

    #[test]
    fn test_transactions_for_ledger_request_uri_with_base_url() {
        let req = for_ledger(888).with_include_failed(true);
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri.to_string().starts_with(
            "https://horizon.stellar.org/some/non/host/url/ledgers/888/transactions?"
        ));
    }

    #[test]
    fn test_transactions_for_claimable_balance_request_uri() {
        let claimable_balance_id =
            "00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9";
        let req = for_claimable_balance(claimable_balance_id).with_include_failed(true);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances/00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9/transactions?"));
    }

    #[test]
    fn test_transactions_for_claimable_balance_request_uri_with_base_url() {
        let claimable_balance_id =
            "00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9";
        let req = for_claimable_balance(claimable_balance_id).with_include_failed(true);
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/claimable_balances/00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9/transactions?"));
    }

    #[test]
    fn test_transactions_for_liquidity_pool_request_uri() {
        let expected_uri = "https://horizon.stellar.org/liquidity_pools/006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207/transactions?";

        let req =
            for_liquidity_pool("006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207");
        let uri = req.uri(&host()).unwrap();

        assert_eq!(expected_uri, uri.as_str());
    }

    #[test]
    fn test_transactions_for_liquidity_pool_request_uri_with_base_url() {
        let expected_uri = "https://horizon.stellar.org/some/non/host/url/liquidity_pools/006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207/transactions?";

        let req =
            for_liquidity_pool("006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207");
        let uri = req.uri(&&base_url()).unwrap();

        assert_eq!(expected_uri, uri.as_str());
    }
}
