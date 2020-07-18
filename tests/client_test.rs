use futures::stream::StreamExt;
use stellar_horizon::api;
use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
use stellar_horizon::request::{Order, PageRequest};

fn new_client() -> HorizonHttpClient {
    HorizonHttpClient::new_from_str("https://horizon-testnet.stellar.org").unwrap()
}

#[tokio::test]
async fn test_root() {
    let client = new_client();
    let response = client.request(api::root::root()).await.unwrap();
    assert!(!response.horizon_version.is_empty());
}

#[tokio::test]
async fn test_single_ledger() {
    let client = new_client();
    let root = client.request(api::root::root()).await.unwrap();
    let response = client
        .request(api::ledgers::single(root.history_latest_ledger))
        .await
        .unwrap();
    assert_eq!(root.history_latest_ledger, response.sequence);
}

#[tokio::test]
async fn test_all_ledgers() {
    let client = new_client();
    let req = api::ledgers::all()
        .with_order(&Order::Descending)
        .with_limit(7);
    let response = client.request(req).await.unwrap();
    assert_eq!(7, response.records().len());
}

#[tokio::test]
async fn test_stream_all_ledgers() {
    let client = new_client();
    let req = api::ledgers::all().with_order(&Order::Descending);
    let mut stream = client.stream(req).unwrap().take(10);
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().paging_token.is_empty());
    }
}

#[tokio::test]
async fn test_stream() {
    let client = new_client();
    let mut stream = client.stream(api::trades::all()).unwrap().take(10);
    let mut count = 0;
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().paging_token.is_empty());
        count += 1;
    }
    assert_eq!(10, count);
}
