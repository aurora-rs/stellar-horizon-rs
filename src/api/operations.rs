use crate::api::Join;
use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources::{self, ClaimableBalanceId, LedgerId};
use stellar_base::PublicKey;
use url::Url;

use super::{accounts, claimable_balances, ledgers, liquidity_pools, transactions};

pub(crate) const API_PATH: &str = "operations";

/// Creates a request to retrieve all operations.
pub fn all() -> AllOperationsRequest {
    Default::default()
}

/// Creates a request to retrieve a single operation.
pub fn single<S>(operation_id: S) -> SingleOperationRequest
where
    S: Into<String>,
{
    SingleOperationRequest {
        operation_id: operation_id.into(),
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
pub fn for_ledger(ledger: LedgerId) -> OperationsForLedgerRequest {
    OperationsForLedgerRequest {
        ledger,
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a transaction's operations.
pub fn for_transaction<S>(tx_id: S) -> OperationsForTransactionRequest
where
    S: Into<String>,
{
    OperationsForTransactionRequest {
        tx_id: tx_id.into(),
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a operations related to a claimable balance.
pub fn for_claimbable_balance<S>(claimable_balance_id: S) -> OperationsForClaimableBalanceRequest
where
    S: Into<String>,
{
    OperationsForClaimableBalanceRequest {
        claimable_balance_id: claimable_balance_id.into(),
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve the operations associated with a liquidity pool.
pub fn for_liquidity_pool<S>(liquidity_pool_id: S) -> OperationsForLiquidityPoolRequest
where
    S: Into<String>,
{
    OperationsForLiquidityPoolRequest {
        liquidity_pool_id: liquidity_pool_id.into(),
        include_failed: None,
        join: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

impl AllOperationsRequest {
    impl_include_failed!();
    impl_join!();
}

impl SingleOperationRequest {
    impl_join!();
}

impl OperationsForAccountRequest {
    impl_include_failed!();
    impl_join!();
}

impl OperationsForLedgerRequest {
    impl_include_failed!();
    impl_join!();
}

impl OperationsForTransactionRequest {
    impl_include_failed!();
    impl_join!();
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
    ledger: LedgerId,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request a transaction operations.
#[derive(Debug, Clone)]
pub struct OperationsForTransactionRequest {
    tx_id: String,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request operations related to a claimable balance.
#[derive(Debug, Clone)]
pub struct OperationsForClaimableBalanceRequest {
    claimable_balance_id: ClaimableBalanceId,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request operations associated with a liquidity pool.
#[derive(Debug, Clone)]
pub struct OperationsForLiquidityPoolRequest {
    liquidity_pool_id: String,
    include_failed: Option<bool>,
    join: Option<Join>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllOperationsRequest {
    type Response = Page<resources::Operation>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        base_url = base_url
            .append_include_failed(&self.include_failed)
            .appen_join(&self.join);
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllOperationsRequest);

impl StreamRequest for AllOperationsRequest {
    type Resource = resources::Operation;
}

impl Request for SingleOperationRequest {
    type Response = resources::Operation;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH, self.operation_id.as_str()]);
        }
        Ok(base_url.appen_join(&self.join))
    }
}

impl Request for OperationsForAccountRequest {
    type Response = Page<resources::Operation>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[accounts::API_PATH, self.account_id.as_str(), API_PATH]);
        }
        base_url = base_url.append_include_failed(&self.include_failed);
        base_url = base_url.appen_join(&self.join);
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(OperationsForAccountRequest);

impl StreamRequest for OperationsForAccountRequest {
    type Resource = resources::Operation;
}

impl Request for OperationsForLedgerRequest {
    type Response = Page<resources::Operation>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            let ledger = self.ledger.to_string();
            segments.extend(&[ledgers::API_PATH, ledger.as_str(), API_PATH]);
        }
        base_url = base_url.append_include_failed(&self.include_failed);
        base_url = base_url.appen_join(&self.join);
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(OperationsForLedgerRequest);

impl StreamRequest for OperationsForLedgerRequest {
    type Resource = resources::Operation;
}

impl Request for OperationsForTransactionRequest {
    type Response = Page<resources::Operation>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[transactions::API_PATH, self.tx_id.as_str(), API_PATH]);
        }
        base_url = base_url.append_include_failed(&self.include_failed);
        base_url = base_url.appen_join(&self.join);
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(OperationsForTransactionRequest);

impl OperationsForClaimableBalanceRequest {
    impl_include_failed!();
    impl_join!();
}

impl Request for OperationsForClaimableBalanceRequest {
    type Response = Page<resources::Operation>;

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

        let base_url = base_url
            .append_include_failed(&self.include_failed)
            .appen_join(&self.join)
            .append_pagination_params(self);
        Ok(base_url)
    }
}

impl_page_request!(OperationsForClaimableBalanceRequest);

impl StreamRequest for OperationsForClaimableBalanceRequest {
    type Resource = resources::Operation;
}

impl OperationsForLiquidityPoolRequest {
    impl_include_failed!();
    impl_join!();
}

impl Request for OperationsForLiquidityPoolRequest {
    type Response = Page<resources::Operation>;

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
        base_url = base_url
            .append_include_failed(&self.include_failed)
            .appen_join(&self.join)
            .append_pagination_params(self);
        Ok(base_url)
    }
}

impl_page_request!(OperationsForLiquidityPoolRequest);

impl StreamRequest for OperationsForLiquidityPoolRequest {
    type Resource = resources::Operation;
}

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

    fn base_url() -> Url {
        "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap()
    }

    #[test]
    fn test_all_operations_request_uri() {
        let req = all()
            .with_include_failed(true)
            .with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/operations?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"true".to_string()), query.get("include_failed"));
    }

    #[test]
    fn test_all_operations_request_uri_with_base_url() {
        let req = all()
            .with_include_failed(true)
            .with_join(Join::Transactions);
        let uri = req.uri(&base_url()).unwrap();
        println!("LNK: {}", uri.to_string());
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/operations?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"true".to_string()), query.get("include_failed"));
    }

    #[test]
    fn test_single_operation_request_uri() {
        let req = single("8181").with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/operations/8181?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"transactions".to_string()), query.get("join"));
    }

    #[test]
    fn test_single_operation_request_uri_with_base_url() {
        let req = single("8181").with_join(Join::Transactions);
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/operations/8181?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"transactions".to_string()), query.get("join"));
    }

    #[test]
    fn test_operation_for_account_request_uri() {
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
            .starts_with("https://horizon.stellar.org/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/operations?"));
    }

    #[test]
    fn test_operation_for_account_request_uri_with_base_url() {
        let pk =
            PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
                .unwrap();
        let req = for_account(&pk)
            .with_cursor("now")
            .with_join(Join::Transactions)
            .with_include_failed(true);
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/operations?"));
    }

    #[test]
    fn test_operation_for_ledger_request_uri() {
        let req = for_ledger(888)
            .with_include_failed(true)
            .with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers/888/operations?"));
    }

    #[test]
    fn test_operation_for_ledger_request_uri_with_base_url() {
        let req = for_ledger(888)
            .with_include_failed(true)
            .with_join(Join::Transactions);
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/ledgers/888/operations?"));
    }

    #[test]
    fn test_operation_for_transaction_request_uri() {
        let req =
            for_transaction("715ffb63673a4ee9b84d4b60924b3e141b34fe3777697f35bad6d4b990524ca2")
                .with_include_failed(true)
                .with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/transactions/715ffb63673a4ee9b84d4b60924b3e141b34fe3777697f35bad6d4b990524ca2/operations?"));
    }

    #[test]
    fn test_operation_for_transaction_request_uri_with_base_url() {
        let req =
            for_transaction("715ffb63673a4ee9b84d4b60924b3e141b34fe3777697f35bad6d4b990524ca2")
                .with_include_failed(true)
                .with_join(Join::Transactions);
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/transactions/715ffb63673a4ee9b84d4b60924b3e141b34fe3777697f35bad6d4b990524ca2/operations?"));
    }

    #[test]
    fn test_operation_for_claimable_balance_request_uri() {
        let req = for_claimbable_balance(
            "00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9",
        )
        .with_include_failed(true)
        .with_join(Join::Transactions);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances/00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9/operations?"));
    }

    #[test]
    fn test_operation_for_claimable_balance_request_uri_with_base_url() {
        let req = for_claimbable_balance(
            "00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9",
        )
        .with_include_failed(true)
        .with_join(Join::Transactions);
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/claimable_balances/00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9/operations?"));
    }

    #[test]
    fn test_operation_for_liquidity_pool_request_uri() {
        let expected_uri = "https://horizon.stellar.org/liquidity_pools/6d30e1f5721962d8bad07d90c606a3963ddbe23c8751cdbdc87224d188f4593c/operations?";
        let liquidity_pool_id = "6d30e1f5721962d8bad07d90c606a3963ddbe23c8751cdbdc87224d188f4593c";

        let req = for_liquidity_pool(liquidity_pool_id);
        let uri = req.uri(&host()).unwrap();
        assert_eq!(expected_uri, uri.as_str());
    }

    #[test]
    fn test_operation_for_liquidity_pool_request_uri_with_base_url() {
        let expected_uri = "https://horizon.stellar.org/some/non/host/url/liquidity_pools/6d30e1f5721962d8bad07d90c606a3963ddbe23c8751cdbdc87224d188f4593c/operations?";
        let liquidity_pool_id = "6d30e1f5721962d8bad07d90c606a3963ddbe23c8751cdbdc87224d188f4593c";

        let req = for_liquidity_pool(liquidity_pool_id);
        let uri = req.uri(&base_url()).unwrap();
        assert_eq!(expected_uri, uri.as_str());
    }
}
