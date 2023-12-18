use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources::{self, LedgerId};
use stellar_base::PublicKey;
use url::Url;

use super::{accounts, ledgers};

pub(crate) const API_PATH: &str = "effects";

/// Create a request to retrieve all effects.
pub fn all() -> AllEffectsRequest {
    Default::default()
}

/// Create a request to retrieve effects for a transaction.
pub fn for_transaction<S>(tx_hash: S) -> EffectsForTransactionRequest
where
    S: Into<String>,
{
    EffectsForTransactionRequest {
        tx_hash: tx_hash.into(),
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Create a request to retrieve effects for an operation.
pub fn for_operation<S>(operation_id: S) -> EffectsForOperationRequest
where
    S: Into<String>,
{
    EffectsForOperationRequest {
        operation_id: operation_id.into(),
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Create a request to retrieve effects for a ledger.
pub fn for_ledger(ledger: LedgerId) -> EffectsForLedgerRequest {
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

/// Create a request to retrieve effects for an operation.
pub fn for_liquidity_pool<S>(liquidity_pool_id: S) -> EffectsForLiquidityPoolRequest
where
    S: Into<String>,
{
    EffectsForLiquidityPoolRequest {
        liquidity_pool_id: liquidity_pool_id.into(),
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
    ledger: LedgerId,
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

/// Request effects for a liquidity pool.
#[derive(Debug, Clone)]
pub struct EffectsForLiquidityPoolRequest {
    liquidity_pool_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllEffectsRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllEffectsRequest);

impl StreamRequest for AllEffectsRequest {
    type Resource = resources::Effect;
}

impl Request for EffectsForLedgerRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            let ledger = self.ledger.to_string();
            segments.extend(&[ledgers::API_PATH, ledger.as_str(), API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForLedgerRequest);

impl StreamRequest for EffectsForLedgerRequest {
    type Resource = resources::Effect;
}

impl Request for EffectsForTransactionRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["transactions", self.tx_hash.as_str(), API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForTransactionRequest);

impl Request for EffectsForOperationRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["operations", self.operation_id.as_str(), API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForOperationRequest);

impl Request for EffectsForAccountRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[accounts::API_PATH, self.account_id.as_str(), API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForAccountRequest);

impl StreamRequest for EffectsForAccountRequest {
    type Resource = resources::Effect;
}

impl Request for EffectsForLiquidityPoolRequest {
    type Response = Page<resources::Effect>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["liquidity_pools", self.liquidity_pool_id.as_str(), API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(EffectsForLiquidityPoolRequest);

impl StreamRequest for EffectsForLiquidityPoolRequest {
    type Resource = resources::Effect;
}

#[cfg(test)]
mod tests {
    use super::{all, for_account, for_ledger, for_operation, for_transaction};
    use crate::api::effects::for_liquidity_pool;
    use crate::request::{Order, PageRequest, Request};
    use std::collections::HashMap;
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
    fn test_all_effects_request_uri() {
        let req = all().with_cursor("now");
        let uri = req.uri(&host()).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/effects?"));
        assert_eq!(Some(&"now".to_string()), query.get("cursor"));
    }

    #[test]
    fn test_all_effects_request_uri_with_base_url() {
        let req = all().with_cursor("now");
        let uri = req.uri(&base_url()).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/effects?"));
        assert_eq!(Some(&"now".to_string()), query.get("cursor"));
    }

    #[test]
    fn test_effects_for_account_request_uri() {
        let pk =
            PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
                .unwrap();
        let req = for_account(&pk).with_order(&Order::Ascending);
        let uri = req.uri(&host()).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/effects?"));
        assert_eq!(Some(&"asc".to_string()), query.get("order"));
    }

    #[test]
    fn test_effects_for_account_request_uri_with_base_url() {
        let pk =
            PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
                .unwrap();
        let req = for_account(&pk).with_order(&Order::Ascending);
        let uri = req.uri(&base_url()).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/effects?"));
        assert_eq!(Some(&"asc".to_string()), query.get("order"));
    }

    #[test]
    fn test_effects_for_ledger_request_uri() {
        let req = for_ledger(123).with_order(&Order::Ascending);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers/123/effects?"));
    }

    #[test]
    fn test_effects_for_ledger_request_uri_with_base_url() {
        let req = for_ledger(123).with_order(&Order::Ascending);
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/ledgers/123/effects?"));
    }

    #[test]
    fn test_effects_for_operation_request_uri() {
        let req = for_operation("12345");
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/operations/12345/effects?"));
    }

    #[test]
    fn test_effects_for_operation_request_uri_with_base_url() {
        let req = for_operation("12345");
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri.to_string().starts_with(
            "https://horizon.stellar.org/some/non/host/url/operations/12345/effects?"
        ));
    }

    #[test]
    fn test_effects_for_transaction_request_uri() {
        let req =
            for_transaction("23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e");
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/transactions/23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e/effects?"));
    }

    #[test]
    fn test_effects_for_transaction_request_uri_with_base_url() {
        let req =
            for_transaction("23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e");
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/transactions/23bf920c4a000b78268589df224c1ba4c883a905687f5a5b3bdba721ee1f481e/effects?"));
    }

    #[test]
    fn test_effects_for_liquidity_pool_request_uri() {
        let expected_uri = "https://horizon.stellar.org/liquidity_pools/006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207/effects?";

        let req =
            for_liquidity_pool("006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207");
        let uri = req.uri(&host()).unwrap();

        assert_eq!(expected_uri, uri.as_str());
    }

    #[test]
    fn test_effects_for_liquidity_pool_request_uri_with_base_url() {
        let expected_uri = "https://horizon.stellar.org/some/non/host/url/liquidity_pools/006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207/effects?";

        let req =
            for_liquidity_pool("006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207");
        let uri = req.uri(&base_url()).unwrap();

        assert_eq!(expected_uri, uri.as_str());
    }
}
