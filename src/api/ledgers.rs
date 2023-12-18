use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources::{self, LedgerId};
use url::Url;

pub(crate) const API_PATH: &str = "ledgers";

/// Creates a request to retrieve a single ledger.
pub fn single(ledger_sequence: LedgerId) -> SingleLedgerRequest {
    SingleLedgerRequest { ledger_sequence }
}

/// Creates a request to retrieve all ledgers.
pub fn all() -> AllLedgersRequest {
    AllLedgersRequest {
        limit: None,
        cursor: None,
        order: None,
    }
}

/// Request a single ledger.
#[derive(Debug, Clone)]
pub struct SingleLedgerRequest {
    ledger_sequence: LedgerId,
}

/// Request all ledgers.
#[derive(Debug, Clone)]
pub struct AllLedgersRequest {
    limit: Option<u64>,
    cursor: Option<String>,
    order: Option<Order>,
}

impl Request for SingleLedgerRequest {
    type Response = resources::Ledger;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            let ledger = self.ledger_sequence.to_string();
            segments.extend(&[API_PATH, ledger.as_str()]);
        }
        Ok(base_url)
    }
}

impl Request for AllLedgersRequest {
    type Response = Page<resources::Ledger>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&[API_PATH]);
        }
        Ok(base_url.append_pagination_params(self))
    }
}

impl_page_request!(AllLedgersRequest);

impl StreamRequest for AllLedgersRequest {
    type Resource = resources::Ledger;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Request;
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
    fn test_all_ledgers_request_uri() {
        let req = all();
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers?"));
    }

    #[test]
    fn test_all_ledgers_request_uri_with_non_host_url() {
        let req = all();
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/ledgers?"));
    }

    #[test]
    fn test_single_ledger_request_uri() {
        let req = single(888);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers/888"));
    }

    #[test]
    fn test_single_ledger_request_uri_with_non_host_url() {
        let req = single(888);
        let uri = req.uri(&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/ledgers/888"));
    }
}
