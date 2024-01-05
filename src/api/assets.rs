use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, UrlPageRequestExt};
use crate::resources;
use stellar_base::asset::{Asset, CreditAsset};
use stellar_base::crypto::PublicKey;
use url::Url;

pub(crate) const API_PATH: &str = "assets";

/// Creates a request to list all assets issued on the network.
pub fn all() -> AllAssetsRequest {
    AllAssetsRequest {
        asset_code: None,
        asset_issuer: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Request all assets issued on the network.
#[derive(Debug, Clone)]
pub struct AllAssetsRequest {
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl AllAssetsRequest {
    /// Filter assets by asset code.
    pub fn with_asset_code<S>(mut self, code: S) -> AllAssetsRequest
    where
        S: Into<String>,
    {
        self.asset_code = Some(code.into());
        self
    }

    /// Filter assets by issuer.
    pub fn with_asset_issuer(mut self, issuer: &PublicKey) -> AllAssetsRequest {
        self.asset_issuer = Some(issuer.account_id());
        self
    }
}

impl Request for AllAssetsRequest {
    type Response = Page<resources::AssetStat>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        {
            let mut query = base_url.query_pairs_mut();
            if let Some(asset_code) = &self.asset_code {
                query.append_pair("asset_code", asset_code);
            }
            if let Some(asset_issuer) = &self.asset_issuer {
                query.append_pair("asset_issuer", asset_issuer);
            }
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllAssetsRequest);

pub(crate) fn credit_asset_to_string(asset: &CreditAsset) -> String {
    let code = asset.code();
    let issuer = asset.issuer().account_id();
    format!("{}:{}", code, issuer)
}

pub(crate) fn asset_to_string(asset: &Asset) -> String {
    match asset {
        Asset::Native => "native".to_string(),
        Asset::Credit(credit) => credit_asset_to_string(credit),
    }
}

#[cfg(test)]
mod tests {
    use super::all;
    use crate::request::{Order, PageRequest, Request};
    use std::collections::HashMap;
    use stellar_base::crypto::PublicKey;
    use url::Url;

    #[test]
    fn test_all_assets_request_uri() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org".parse().unwrap();
        let req = all().with_asset_code("CODE").with_asset_issuer(&pk);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/assets?"));
        assert_eq!(Some(&"CODE".to_string()), query.get("asset_code"));
        assert_eq!(Some(&pk.account_id()), query.get("asset_issuer"));
    }

    #[test]
    fn test_all_assets_request_uri_with_base_url() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap();
        let req = all().with_asset_code("CODE").with_asset_issuer(&pk);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/assets?"));
        assert_eq!(Some(&"CODE".to_string()), query.get("asset_code"));
        assert_eq!(Some(&pk.account_id()), query.get("asset_issuer"));
    }

    #[test]
    fn test_all_assets_request_uri_with_page() {
        let host: Url = "https://horizon.stellar.org".parse().unwrap();
        let req = all()
            .with_cursor("now")
            .with_order(&Order::Descending)
            .with_limit(100);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/assets?"));
        assert_eq!(Some(&"100".to_string()), query.get("limit"));
        assert_eq!(Some(&"now".to_string()), query.get("cursor"));
        assert_eq!(Some(&"desc".to_string()), query.get("order"));
    }

    #[test]
    fn test_all_assets_request_uri_with_page_with_base_url() {
        let host: Url = "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap();
        let req = all()
            .with_cursor("now")
            .with_order(&Order::Descending)
            .with_limit(100);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/assets?"));
        assert_eq!(Some(&"100".to_string()), query.get("limit"));
        assert_eq!(Some(&"now".to_string()), query.get("cursor"));
        assert_eq!(Some(&"desc".to_string()), query.get("order"));
    }
}
