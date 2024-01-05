use stellar_base::{Asset, PublicKey};
use url::Url;

use crate::api::assets::asset_to_string;
use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, UrlPageRequestExt};
use crate::resources;

pub(crate) const API_PATH: &str = "liquidity_pools";

/// Creates a request to retrieve all liquidity pools.
pub fn all() -> AllLiquidityPoolsRequest {
    AllLiquidityPoolsRequest {
        reserves: None,
        account: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a single liquidity pool.
pub fn single(id: String) -> SingleLiquidityPoolRequest {
    SingleLiquidityPoolRequest { id }
}

/// Request all liquidity pools.
#[derive(Debug, Clone)]
pub struct AllLiquidityPoolsRequest {
    reserves: Option<Vec<Asset>>,
    account: Option<String>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl AllLiquidityPoolsRequest {
    /// Updates the request to filter results by reserves.
    pub fn with_reserves(mut self, reserves: Vec<Asset>) -> Self {
        self.reserves = Some(reserves);
        self
    }

    /// Updates the request to filter results by account.
    pub fn with_account(mut self, account: &PublicKey) -> Self {
        self.account = Some(account.account_id());
        self
    }
}

impl Request for AllLiquidityPoolsRequest {
    type Response = Page<resources::LiquidityPool>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        {
            let mut query = base_url.query_pairs_mut();

            if let Some(reserves) = self.reserves.as_ref() {
                let reserve_str = reserves
                    .iter()
                    .map(asset_to_string)
                    .collect::<Vec<String>>()
                    .join(",");
                if !reserve_str.is_empty() {
                    query.append_pair("reserves", &reserve_str);
                }
            }

            if let Some(account_id) = self.account.as_ref() {
                query.append_pair("account", account_id);
            }
        }

        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllLiquidityPoolsRequest);

/// Request a single liquidity pool.
#[derive(Debug, Clone)]
pub struct SingleLiquidityPoolRequest {
    id: String,
}

impl Request for SingleLiquidityPoolRequest {
    type Response = resources::LiquidityPool;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH, self.id.as_str()]);
        }
        Ok(base_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Request;
    use std::collections::HashMap;
    use stellar_base::asset::Asset;
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
    fn test_all_liquidity_pools_request_uri() {
        let account =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let issuer =
            PublicKey::from_account_id("GDPFNXAJ6R37LBQ6QYVKGBVW5ZA4QXPFJYKQUHPJSALXCUBQ7I5K6YFN")
                .unwrap();

        let reserves = vec![
            Asset::new_native(),
            Asset::new_credit("BUSD", issuer).unwrap(),
        ];

        let req = all().with_account(&account).with_reserves(reserves);

        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/liquidity_pools?"));

        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();

        let expected_reserves =
            "native,BUSD:GDPFNXAJ6R37LBQ6QYVKGBVW5ZA4QXPFJYKQUHPJSALXCUBQ7I5K6YFN".to_string();
        assert_eq!(Some(&expected_reserves), query.get("reserves"));
        assert_eq!(Some(&account.account_id()), query.get("account"));
    }

    #[test]
    fn test_all_liquidity_pools_request_uri_with_base_url() {
        let account =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let issuer =
            PublicKey::from_account_id("GDPFNXAJ6R37LBQ6QYVKGBVW5ZA4QXPFJYKQUHPJSALXCUBQ7I5K6YFN")
                .unwrap();

        let reserves = vec![
            Asset::new_native(),
            Asset::new_credit("BUSD", issuer).unwrap(),
        ];

        let req = all().with_account(&account).with_reserves(reserves);

        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/liquidity_pools?"));

        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();

        let expected_reserves =
            "native,BUSD:GDPFNXAJ6R37LBQ6QYVKGBVW5ZA4QXPFJYKQUHPJSALXCUBQ7I5K6YFN".to_string();
        assert_eq!(Some(&expected_reserves), query.get("reserves"));
        assert_eq!(Some(&account.account_id()), query.get("account"));
    }

    #[test]
    fn test_single_liquidity_pools_request_uri() {
        let liquidity_pool_id =
            "67260c4c1807b262ff851b0a3fe141194936bb0215b2f77447f1df11998eabb9".to_string();
        let expected_uri = format!(
            "https://horizon.stellar.org/liquidity_pools/{}",
            liquidity_pool_id
        );

        let req = single(liquidity_pool_id.clone());

        let uri = req.uri(&host()).unwrap();
        assert_eq!(expected_uri, uri.to_string());
    }

    #[test]
    fn test_single_liquidity_pools_request_uri_with_base_url() {
        let liquidity_pool_id =
            "67260c4c1807b262ff851b0a3fe141194936bb0215b2f77447f1df11998eabb9".to_string();
        let expected_uri = format!(
            "https://horizon.stellar.org/some/non/host/url/liquidity_pools/{}",
            liquidity_pool_id
        );

        let req = single(liquidity_pool_id.clone());

        let uri = req.uri(&base_url()).unwrap();
        assert_eq!(expected_uri, uri.to_string());
    }
}
