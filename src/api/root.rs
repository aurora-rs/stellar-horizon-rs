use crate::api::Page;
use crate::error::{Error, Result};
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use chrono::{DateTime, Duration, Utc};
use std::convert::TryInto;
use stellar_base::amount::{Amount, Stroops};
use stellar_base::asset::{Asset, CreditAsset};
use stellar_base::crypto::PublicKey;
use stellar_base::error::Error as StellarBaseError;
use url::Url;

pub fn root() -> RootRequest {
    RootRequest {}
}

#[derive(Debug, Copy, Clone)]
pub struct RootRequest {}

impl Request for RootRequest {
    type Response = resources::Root;

    fn uri(&self, host: &Url) -> Result<Url> {
        Ok(host.clone())
    }
}
