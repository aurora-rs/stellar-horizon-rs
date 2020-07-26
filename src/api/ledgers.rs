use crate::error::Result;
use crate::page::Page;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use url::Url;

/// Creates a request to retrieve a single ledger.
pub fn single(ledger_sequence: i32) -> SingleLedgerRequest {
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
    ledger_sequence: i32,
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

    fn uri(&self, host: &Url) -> Result<Url> {
        let path = format!("/ledgers/{}", self.ledger_sequence);
        Ok(host.join(&path)?)
    }
}

impl Request for AllLedgersRequest {
    type Response = Page<resources::Ledger>;

    fn uri(&self, host: &Url) -> Result<Url> {
        let url = host.join("/ledgers")?;
        Ok(url.append_pagination_params(self))
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

    #[test]
    fn test_all_ledgers_request_uri() {
        let req = all();
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers?"));
    }

    #[test]
    fn test_single_ledger_request_uri() {
        let req = single(888);
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/ledgers/888"));
    }
}
