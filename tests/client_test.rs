use futures::stream::StreamExt;
use stellar_base::{KeyPair, Network};
use stellar_horizon::api;
use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
use stellar_horizon::request::{Order, PageRequest};

fn new_client() -> HorizonHttpClient {
    HorizonHttpClient::new_from_str("https://horizon-testnet.stellar.org").unwrap()
}

fn new_root_key() -> KeyPair {
    KeyPair::from_network(&Network::new_test()).unwrap()
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
async fn test_single_account() {
    let client = new_client();
    let root_key = new_root_key();
    let req = api::accounts::single(root_key.public_key());
    let response = client.request(req).await.unwrap();
    assert_eq!(root_key.public_key().account_id(), response.paging_token);
}
