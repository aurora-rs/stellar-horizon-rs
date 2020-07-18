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
    assert_eq!(7, response.records.len());
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

#[tokio::test]
async fn test_all_transactions() {
    let client = new_client();
    let req = api::transactions::all()
        .with_order(&Order::Descending)
        .with_limit(5);
    let response = client.request(req).await.unwrap();
    assert_eq!(response.records.len(), 5);
}

#[tokio::test]
async fn test_stream_all_transactions() {
    let client = new_client();
    let req = api::transactions::all().with_order(&Order::Descending);
    let mut stream = client.stream(req).unwrap().take(10);
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().paging_token.is_empty());
    }
}

#[tokio::test]
async fn test_transactions_for_account() {
    let client = new_client();
    let root_key = new_root_key();
    let req = api::transactions::for_account(root_key.public_key())
        .with_order(&Order::Descending)
        .with_limit(5);
    let response = client.request(req).await.unwrap();
    assert_eq!(response.records.len(), 5);
}

#[tokio::test]
async fn test_transactions_for_ledger() {
    let client = new_client();
    let root = client.request(api::root::root()).await.unwrap();
    let req = api::transactions::for_ledger(root.history_latest_ledger as u32);
    let _response = client.request(req).await.unwrap();
}

#[tokio::test]
async fn test_all_trades() {
    let client = new_client();
    let req = api::trades::all()
        .with_order(&Order::Descending)
        .with_limit(5);
    let response = client.request(req).await.unwrap();
    assert_eq!(response.records.len(), 5);
}

#[tokio::test]
async fn test_stream_all_trades() {
    let client = new_client();
    let req = api::trades::all().with_order(&Order::Descending);
    let mut stream = client.stream(req).unwrap().take(10);
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().paging_token.is_empty());
    }
}

#[tokio::test]
async fn test_trades_for_account() {
    let client = new_client();
    let root_key = new_root_key();
    let req = api::trades::for_account(root_key.public_key())
        .with_order(&Order::Descending)
        .with_limit(5);
    let response = client.request(req).await.unwrap();
    assert!(response.records.is_empty());
}

#[tokio::test]
async fn test_data_for_account() {
    let client = new_client();
    let root_key = new_root_key();
    let req = api::data::for_account(root_key.public_key(), "FooBar");
    let response = client.request(req).await;
    assert!(response.is_err());
}
