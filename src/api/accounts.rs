use crate::api::assets::credit_asset_to_string;
use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, UrlPageRequestExt};
use crate::resources;
use stellar_base::asset::CreditAsset;
use stellar_base::crypto::PublicKey;
use url::Url;

pub(crate) const API_PATH: &str = "accounts";

/// Creates a request to retrieve a single account.
pub fn single(public_key: &PublicKey) -> SingleAccountRequest {
    let account_id = public_key.account_id();
    SingleAccountRequest { account_id }
}

/// Creates a request to retrieve all accounts.
pub fn all() -> AllAccountsRequest {
    Default::default()
}

impl AllAccountsRequest {
    /// Updates the request to filter results by signer.
    pub fn with_signer(mut self, signer: &PublicKey) -> AllAccountsRequest {
        self.signer = Some(signer.account_id());
        self
    }

    /// Updates the request to filter results by trust line to asset.
    pub fn with_trusted_asset(mut self, asset: CreditAsset) -> AllAccountsRequest {
        self.asset = Some(asset);
        self
    }

    /// Updates the request to filter results by sponsor.
    pub fn with_sponsor(mut self, sponsor: &PublicKey) -> AllAccountsRequest {
        self.sponsor = Some(sponsor.account_id());
        self
    }
}

/// Request a single account.
#[derive(Debug, Clone)]
pub struct SingleAccountRequest {
    account_id: String,
}

/// Request all accounts.
#[derive(Debug, Clone, Default)]
pub struct AllAccountsRequest {
    asset: Option<CreditAsset>,
    signer: Option<String>,
    sponsor: Option<String>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for SingleAccountRequest {
    type Response = resources::Account;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH, self.account_id.as_str()]);
        }
        Ok(base_url)
    }
}

impl Request for AllAccountsRequest {
    type Response = Page<resources::Account>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        if let Some(signer) = self.signer.as_ref() {
            base_url = base_url.append_query_param("signer", signer);
        }
        if let Some(asset) = self.asset.as_ref() {
            base_url = base_url.append_query_param("asset", &credit_asset_to_string(asset));
        }
        if let Some(sponsor) = self.sponsor.as_ref() {
            base_url = base_url.append_query_param("sponsor", sponsor);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllAccountsRequest);

#[cfg(test)]
mod tests {
    use super::{all, single};
    use crate::request::Request;
    use std::collections::HashMap;
    use stellar_base::crypto::PublicKey;
    use url::Url;

    #[test]
    fn test_single_request_uri() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org".parse().unwrap();
        let req = single(&pk);
        let uri = req.uri(&host).unwrap();
        assert_eq!(
            "https://horizon.stellar.org/accounts/GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA",
            uri.to_string()
        );
    }

    #[test]
    fn test_single_request_uri_with_base_url() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap();
        let req = single(&pk);
        let uri = req.uri(&host).unwrap();
        assert_eq!(
            "https://horizon.stellar.org/some/non/host/url/accounts/GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA",
            uri.to_string()
        );
    }

    #[test]
    fn test_all_with_signer_request_uri() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org".parse().unwrap();
        let req = all().with_signer(&pk);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/accounts?"));
        assert_eq!(Some(&pk.account_id()), query.get("signer"));
    }

    #[test]
    fn test_all_with_signer_request_uri_with_base_url() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap();
        let req = all().with_signer(&pk);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/accounts?"));
        assert_eq!(Some(&pk.account_id()), query.get("signer"));
    }

    #[test]
    fn test_all_with_sponsor_request_uri() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org".parse().unwrap();
        let req = all().with_sponsor(&pk);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/accounts?"));
        assert_eq!(Some(&pk.account_id()), query.get("sponsor"));
    }

    #[test]
    fn test_all_with_sponsor_request_uri_with_base_url() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let host: Url = "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap();
        let req = all().with_sponsor(&pk);
        let uri = req.uri(&host).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/accounts?"));
        assert_eq!(Some(&pk.account_id()), query.get("sponsor"));
    }
}
