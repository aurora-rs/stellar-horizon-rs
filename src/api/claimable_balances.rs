use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, UrlPageRequestExt};
use crate::resources::{self, ClaimableBalanceId};
use stellar_base::asset::Asset;
use stellar_base::crypto::PublicKey;
use url::Url;

pub(crate) const API_PATH: &str = "claimable_balances";

/// Creates a request to retrieve all claimable balances.
pub fn all() -> AllClaimableBalancesRequest {
    AllClaimableBalancesRequest::default()
}

/// Creates a request to retrieve a single offer.
pub fn single(balance_id: ClaimableBalanceId) -> SingleClaimableBalanceRequest {
    SingleClaimableBalanceRequest { balance_id }
}

/// Request all open offers.
#[derive(Debug, Clone, Default)]
pub struct AllClaimableBalancesRequest {
    asset: Option<Asset>,
    claimant: Option<String>,
    sponsor: Option<String>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl AllClaimableBalancesRequest {
    pub fn with_asset(mut self, asset: Asset) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn with_claimant(mut self, claimant: &PublicKey) -> Self {
        self.claimant = Some(claimant.to_string());
        self
    }

    pub fn with_sponsor(mut self, sponsor: &PublicKey) -> Self {
        self.sponsor = Some(sponsor.to_string());
        self
    }
}

impl Request for AllClaimableBalancesRequest {
    type Response = Page<resources::ClaimableBalance>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        if let Some(asset) = self.asset.as_ref() {
            base_url = base_url.append_canonical_asset_params("asset", asset);
        }
        if let Some(claimant) = self.claimant.as_ref() {
            base_url = base_url.append_query_param("claimant", claimant);
        }
        if let Some(sponsor) = self.sponsor.as_ref() {
            base_url = base_url.append_query_param("sponsor", sponsor);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllClaimableBalancesRequest);

/// Request a single offer.
#[derive(Debug, Clone)]
pub struct SingleClaimableBalanceRequest {
    balance_id: ClaimableBalanceId,
}

impl Request for SingleClaimableBalanceRequest {
    type Response = resources::ClaimableBalance;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH, self.balance_id.as_str()]);
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
    fn test_all_claimable_balances_request_uri() {
        let asset = Asset::new_native();
        let claimant =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let sponsor =
            PublicKey::from_account_id("GBLY3AN6XDY2FZMZLGL7YX6ZWI7YC6H3Z2QHQKE6SJ2LDX6UGNPHDNUU")
                .unwrap();

        let req = all()
            .with_asset(asset)
            .with_claimant(&claimant)
            .with_sponsor(&sponsor);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"native".to_string()), query.get("asset"));
        assert_eq!(
            Some(&"GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA".to_string()),
            query.get("claimant")
        );
        assert_eq!(
            Some(&"GBLY3AN6XDY2FZMZLGL7YX6ZWI7YC6H3Z2QHQKE6SJ2LDX6UGNPHDNUU".to_string()),
            query.get("sponsor")
        );
    }

    #[test]
    fn test_all_claimable_balances_request_uri_with_base_url() {
        let asset = Asset::new_native();
        let claimant =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let sponsor =
            PublicKey::from_account_id("GBLY3AN6XDY2FZMZLGL7YX6ZWI7YC6H3Z2QHQKE6SJ2LDX6UGNPHDNUU")
                .unwrap();

        let req = all()
            .with_asset(asset)
            .with_claimant(&claimant)
            .with_sponsor(&sponsor);
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/claimable_balances?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"native".to_string()), query.get("asset"));
        assert_eq!(
            Some(&"GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA".to_string()),
            query.get("claimant")
        );
        assert_eq!(
            Some(&"GBLY3AN6XDY2FZMZLGL7YX6ZWI7YC6H3Z2QHQKE6SJ2LDX6UGNPHDNUU".to_string()),
            query.get("sponsor")
        );
    }

    #[test]
    fn test_single_claimable_balance_request_uri() {
        let req = single(
            "00000000c582697b67cbec7f9ce64f4dc67bfb2bfd26318bb9f964f4d70e3f41f650b1e6".to_string(),
        );
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances/00000000c582697b67cbec7f9ce64f4dc67bfb2bfd26318bb9f964f4d70e3f41f650b1e6"));
    }

    #[test]
    fn test_single_claimable_balance_request_uri_with_url() {
        let req = single(
            "00000000c582697b67cbec7f9ce64f4dc67bfb2bfd26318bb9f964f4d70e3f41f650b1e6".to_string(),
        );
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/claimable_balances/00000000c582697b67cbec7f9ce64f4dc67bfb2bfd26318bb9f964f4d70e3f41f650b1e6"));
    }
}
