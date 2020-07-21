use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, UrlPageRequestExt};
use crate::resources;
use stellar_base::crypto::PublicKey;
use url::Url;

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

    fn uri(&self, host: &Url) -> Result<Url> {
        let mut url = host.join("/assets")?;
        {
            let mut query = url.query_pairs_mut();
            if let Some(asset_code) = &self.asset_code {
                query.append_pair("asset_code", &asset_code);
            }
            if let Some(asset_issuer) = &self.asset_issuer {
                query.append_pair("asset_issuer", &asset_issuer);
            }
        }
        Ok(url.append_pagination_params(self))
    }
}

impl_page_request!(AllAssetsRequest);
