use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use stellar_base::{Asset, PublicKey};
use url::Url;

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

/// Request all trades.
#[derive(Debug, Clone)]
pub struct AllTradesRequest {
    offer_id: Option<String>,
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

impl AllTradesRequest {
    /// Filter trades originating from `offer_id`.
    pub fn with_offer_id(mut self, offer_id: &str) -> AllTradesRequest {
        self.offer_id = Some(offer_id.to_string());
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

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/trades")?;
        if let Some(offer_id) = &self.offer_id {
            url = url.append_query_param("offer_id", offer_id);
        }
        if let Some(asset) = &self.base_asset {
            url = url.append_asset_params(&asset, Some("base_"));
        }
        if let Some(asset) = &self.counter_asset {
            url = url.append_asset_params(&asset, Some("counter_"));
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllTradesRequest);

impl StreamRequest for AllTradesRequest {
    type Resource = resources::Trade;
}

impl Request for TradesForAccountRequest {
    type Response = Page<resources::Trade>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join(&format!("/accounts/{}/trades", self.account_id))?;
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(TradesForAccountRequest);

impl StreamRequest for TradesForAccountRequest {
    type Resource = resources::Trade;
}
