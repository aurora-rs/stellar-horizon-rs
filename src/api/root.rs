use crate::error::Result;
use crate::request::Request;
use crate::resources;
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
