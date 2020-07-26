use crate::api::assets::asset_to_string;
use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, UrlPageRequestExt};
use crate::resources;
use stellar_base::asset::Asset;
use stellar_base::crypto::PublicKey;
use url::Url;

/// Creates a request to retrieve all open offers.
pub fn all() -> AllOffersRequest {
    AllOffersRequest {
        seller: None,
        selling: None,
        buying: None,
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Creates a request to retrieve a single offer.
pub fn single(offer_id: i64) -> SingleOfferRequest {
    SingleOfferRequest { offer_id }
}

/// Creates a request to retrieve the account's offers.
pub fn for_account(account: &PublicKey) -> OffersForAccountRequest {
    OffersForAccountRequest {
        account_id: account.account_id(),
        limit: None,
        cursor: None,
        order: None,
    }
}

impl AllOffersRequest {
    /// Filter by the account id of the offer creator.
    pub fn with_seller(mut self, pk: &PublicKey) -> AllOffersRequest {
        self.seller = Some(pk.account_id());
        self
    }

    /// Filter by the asset being sold.
    pub fn with_selling(mut self, selling: Asset) -> AllOffersRequest {
        self.selling = Some(selling);
        self
    }

    /// Filter by the asset being bought.
    pub fn with_buying(mut self, buying: Asset) -> AllOffersRequest {
        self.buying = Some(buying);
        self
    }
}

/// Request all open offers.
#[derive(Debug, Clone)]
pub struct AllOffersRequest {
    seller: Option<String>,
    selling: Option<Asset>,
    buying: Option<Asset>,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

/// Request a single offer.
#[derive(Debug, Clone)]
pub struct SingleOfferRequest {
    offer_id: i64,
}

/// Request offers for an account.
#[derive(Debug, Clone)]
pub struct OffersForAccountRequest {
    account_id: String,
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for AllOffersRequest {
    type Response = Page<resources::Offer>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/offers")?;
        if let Some(seller) = self.seller.as_ref() {
            url = url.append_query_param("seller", seller);
        }
        if let Some(selling) = self.selling.as_ref() {
            url = url.append_query_param("selling", &asset_to_string(selling));
        }
        if let Some(buying) = self.buying.as_ref() {
            url = url.append_query_param("buying", &asset_to_string(buying));
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllOffersRequest);

impl Request for SingleOfferRequest {
    type Response = resources::Offer;

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.join(&format!("/offers/{}", self.offer_id))?)
    }
}

impl_page_request!(OffersForAccountRequest);

impl Request for OffersForAccountRequest {
    type Response = Page<resources::Offer>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/accounts/{}/offers", self.account_id))?;
        Ok(url.append_pagination_params(self))
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
    fn test_all_offers_request_uri() {
        let req = all()
            .with_seller(&keypair0())
            .with_selling(Asset::new_native())
            .with_buying(credit_asset0())
            .with_limit(10);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/offers?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"10".to_string()), query.get("limit"));
        assert_eq!(
            Some(&"GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623".to_string()),
            query.get("seller")
        );
        assert_eq!(Some(&"native".to_string()), query.get("selling"));
        assert_eq!(
            Some(&"ABCD:GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623".to_string()),
            query.get("buying")
        );
    }

    #[test]
    fn test_single_offer_request_uri() {
        let req = single(123);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/offers/123"));
    }

    #[test]
    fn test_offer_for_account_request_uri() {
        let req = for_account(&keypair0()).with_cursor("now");
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/accounts/GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623/offers?"));
    }
}
