use futures::stream::StreamExt;
use stellar_horizon::api;
use stellar_horizon::client::{HorizonClient, HorizonHttpClient};

#[tokio::test]
async fn test_http_client() {
    let client = HorizonHttpClient::new_from_str("https://horizon-testnet.stellar.org").unwrap();
    let response = client.request(api::root::root()).await.unwrap();
    assert!(!response.horizon_version.is_empty());
}

#[tokio::test]
async fn test_stream() {
    let client = HorizonHttpClient::new_from_str("https://horizon-testnet.stellar.org").unwrap();
    let mut stream = client.stream(api::trades::all()).unwrap().take(10);
    let mut count = 0;
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().paging_token.is_empty());
        count += 1;
    }
    assert_eq!(10, count);
}
