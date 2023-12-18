use crate::error::Result;
use crate::request::Request;
use crate::resources;
use stellar_base::crypto::PublicKey;
use url::Url;

use super::accounts;

pub(crate) const DATA_PATH: &str = "data";

/// Creates a request to retrieve a single data for the account.
pub fn for_account<S: Into<String>>(account: &PublicKey, key: S) -> DataForAccountRequest {
    DataForAccountRequest {
        account_id: account.account_id(),
        key: key.into(),
    }
}

/// Request account data.
#[derive(Debug, Clone)]
pub struct DataForAccountRequest {
    account_id: String,
    key: String,
}

impl Request for DataForAccountRequest {
    type Response = resources::AccountData;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[
                accounts::API_PATH,
                self.account_id.as_str(),
                DATA_PATH,
                self.key.as_str(),
            ]);
        }
        Ok(base_url)
    }
}
