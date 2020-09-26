use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, UrlPageRequestExt};
use crate::resources::{self, ClaimableBalanceId};
use stellar_base::asset::Asset;
use stellar_base::crypto::PublicKey;
use url::Url;

/// Creates a request to retrieve all claimable balances filtered by asset.
pub fn all_by_asset(asset: Asset) -> AllClaimableBalancesRequest {
    AllClaimableBalancesRequest {
        asset: Some(asset),
        ..Default::default()
    }
}

/// Creates a request to retrieve all claimable balances filtered by claimant.
pub fn all_by_claimant(claimant: &PublicKey) -> AllClaimableBalancesRequest {
    AllClaimableBalancesRequest {
        claimant: Some(claimant.account_id()),
        ..Default::default()
    }
}

/// Creates a request to retrieve all claimable balances filtered by sponsor.
pub fn all_by_sponsor(sponsor: &PublicKey) -> AllClaimableBalancesRequest {
    AllClaimableBalancesRequest {
        sponsor: Some(sponsor.account_id()),
        ..Default::default()
    }
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

/// Request a single offer.
#[derive(Debug, Clone)]
pub struct SingleClaimableBalanceRequest {
    balance_id: ClaimableBalanceId,
}

impl Request for AllClaimableBalancesRequest {
    type Response = Page<resources::ClaimableBalance>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/claimable_balances")?;
        if let Some(asset) = self.asset.as_ref() {
            url = url.append_canonical_asset_params("asset", asset);
        }
        if let Some(claimant) = self.claimant.as_ref() {
            url = url.append_query_param("claimant", claimant);
        }
        if let Some(sponsor) = self.sponsor.as_ref() {
            url = url.append_query_param("sponsor", sponsor);
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllClaimableBalancesRequest);

impl Request for SingleClaimableBalanceRequest {
    type Response = resources::ClaimableBalance;

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join(&format!("/claimable_balances/{}", self.balance_id))?)
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

    #[test]
    fn test_all_claimable_balances_by_native_asset_request_uri() {
        let req = all_by_asset(Asset::new_native());
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"native".to_string()), query.get("asset"));
    }

    #[test]
    fn test_all_claimable_balances_by_credit_asset_request_uri() {
        let issuer =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let asset = Asset::new_credit("XYZ", issuer).unwrap();
        let req = all_by_asset(asset);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(
            Some(&"XYZ:GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA".to_string()),
            query.get("asset")
        );
    }

    #[test]
    fn test_all_claimable_balances_by_claimant_request_uri() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let req = all_by_claimant(&pk);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&pk.account_id()), query.get("claimant"));
    }

    #[test]
    fn test_all_claimable_balances_by_sponsor_request_uri() {
        let pk =
            PublicKey::from_account_id("GAYOLLLUIZE4DZMBB2ZBKGBUBZLIOYU6XFLW37GBP2VZD3ABNXCW4BVA")
                .unwrap();
        let req = all_by_sponsor(&pk);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/claimable_balances?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&pk.account_id()), query.get("sponsor"));
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
}
