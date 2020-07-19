use crate::api::Page;
use crate::error::Result;
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
        Ok(host.join("/offers")?)
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
