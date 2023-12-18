use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources::{self, OfferId};
use stellar_base::{Asset, PublicKey};
use url::Url;

use super::accounts;
use super::{liquidity_pools, offers};

pub(crate) const API_PATH: &str = "trades";

/// Creates a request to retrieve all trades.
pub fn all() -> AllTradesRequest {
    AllTradesRequest {
        offer_id: None,
        base_asset: None,
        counter_asset: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve all trades for the account.
pub fn for_account(account: &PublicKey) -> TradesForAccountRequest {
    TradesForAccountRequest {
        account_id: account.account_id(),
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve all trades for an offer.
pub fn for_offer(offer_id: OfferId) -> TradesForOfferRequest {
    TradesForOfferRequest {
        offer_id,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve all trades associated with a liquidity pool.
pub fn for_liquidity_pool<S: Into<String>>(liquidity_pool_id: S) -> TradesForLiquidityPoolRequest {
    TradesForLiquidityPoolRequest {
        liquidity_pool_id: liquidity_pool_id.into(),
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Request all trades.
#[derive(Debug, Clone)]
pub struct AllTradesRequest {
    offer_id: Option<OfferId>,
    base_asset: Option<Asset>,
    counter_asset: Option<Asset>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request trades for account.
#[derive(Debug, Clone)]
pub struct TradesForAccountRequest {
    account_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request trades for an offer.
#[derive(Debug, Clone)]
pub struct TradesForOfferRequest {
    offer_id: OfferId,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request trades associated with a liquidity pool.
#[derive(Debug, Clone)]
pub struct TradesForLiquidityPoolRequest {
    liquidity_pool_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl AllTradesRequest {
    /// Filter trades originating from `offer_id`.
    pub fn with_offer_id(mut self, offer_id: OfferId) -> AllTradesRequest {
        self.offer_id = Some(offer_id);
        self
    }

    /// Filter trades by base asset.
    pub fn with_base_asset(mut self, asset: Asset) -> AllTradesRequest {
        self.base_asset = Some(asset);
        self
    }

    /// Filter trades by counter asset.
    pub fn with_counter_asset(mut self, asset: Asset) -> AllTradesRequest {
        self.counter_asset = Some(asset);
        self
    }
}

impl Request for AllTradesRequest {
    type Response = Page<resources::Trade>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        if let Some(offer_id) = &self.offer_id {
            base_url = base_url.append_query_param("offer_id", &offer_id.to_string());
        }
        if let Some(asset) = &self.base_asset {
            base_url = base_url.append_asset_params(asset, Some("base"));
        }
        if let Some(asset) = &self.counter_asset {
            base_url = base_url.append_asset_params(asset, Some("counter"));
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllTradesRequest);

impl StreamRequest for AllTradesRequest {
    type Resource = resources::Trade;
}

impl Request for TradesForAccountRequest {
    type Response = Page<resources::Trade>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[accounts::API_PATH, &self.account_id.as_str(), API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(TradesForAccountRequest);

impl StreamRequest for TradesForAccountRequest {
    type Resource = resources::Trade;
}

impl Request for TradesForOfferRequest {
    type Response = Page<resources::Trade>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            let offer_id = self.offer_id.to_string();
            segments.extend(&[offers::API_PATH, offer_id.as_str(), API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(TradesForOfferRequest);

impl Request for TradesForLiquidityPoolRequest {
    type Response = Page<resources::Trade>;

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
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(TradesForLiquidityPoolRequest);

impl StreamRequest for TradesForLiquidityPoolRequest {
    type Resource = resources::Trade;
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

    fn keypair0() -> PublicKey {
        PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
            .unwrap()
    }

    fn credit_asset0() -> Asset {
        let issuer = keypair0();
        let code = "ABCD";
        Asset::new_credit(code, issuer).unwrap()
    }

    #[test]
    fn test_all_trades_request_uri() {
        let req = all()
            .with_offer_id(123)
            .with_base_asset(Asset::new_native())
            .with_counter_asset(credit_asset0());
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/trades?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"native".to_string()), query.get("base_asset_type"));
        assert_eq!(
            Some(&"credit_alphanum4".to_string()),
            query.get("counter_asset_type")
        );
    }

    #[test]
    fn test_all_trades_request_uri_with_bae_url() {
        let req = all()
            .with_offer_id(123)
            .with_base_asset(Asset::new_native())
            .with_counter_asset(credit_asset0());
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/trades?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"native".to_string()), query.get("base_asset_type"));
        assert_eq!(
            Some(&"credit_alphanum4".to_string()),
            query.get("counter_asset_type")
        );
    }

    #[test]
    fn test_trades_for_account_request_uri() {
        let req = for_account(&keypair0());
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/trades?"));
    }

    #[test]
    fn test_trades_for_account_request_uri_with_base_url() {
        let req = for_account(&keypair0());
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/trades?"));
    }

    #[test]
    fn test_trades_for_offer_request_uri() {
        let req = for_offer(888);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/offers/888/trades?"));
    }

    #[test]
    fn test_trades_for_offer_request_uri_with_base_url() {
        let req = for_offer(888);
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/offers/888/trades?"));
    }

    #[test]
    fn test_trades_for_liquidity_pool_request_uri() {
        let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";
        let expected_uri = "https://horizon.stellar.org/liquidity_pools/0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16/trades?";

        let req = for_liquidity_pool(liquidity_pool_id);
        let uri = req.uri(&host()).unwrap();
        assert_eq!(expected_uri, uri.as_str());
    }

    #[test]
    fn test_trades_for_liquidity_pool_request_uri_with_base_url() {
        let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";
        let expected_uri = "https://horizon.stellar.org/some/non/host/url/liquidity_pools/0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16/trades?";

        let req = for_liquidity_pool(liquidity_pool_id);
        let uri = req.uri(&&base_url()).unwrap();
        assert_eq!(expected_uri, uri.as_str());
    }
}
