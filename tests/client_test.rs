use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use stellar_base::account::DataValue;
use stellar_base::amount::Amount;
use stellar_base::time_bounds::TimeBounds;
use stellar_base::transaction::MIN_BASE_FEE;
use stellar_base::{Asset, KeyPair, Network, Operation, PublicKey, Transaction};
use stellar_horizon::api;
use stellar_horizon::api::aggregations::Resolution;
use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
use stellar_horizon::request::{Order, PageRequest};
use tokio::stream::StreamExt;

fn new_client() -> HorizonHttpClient {
    HorizonHttpClient::new_from_str("https://horizon.stellar.org").unwrap()
}

fn new_root_key() -> KeyPair {
    KeyPair::from_network(&Network::new_public()).unwrap()
}

fn new_project_key_pair() -> KeyPair {
    let secret_seed = env::var("SECRET_SEED").unwrap();
    KeyPair::from_secret_seed(&secret_seed).unwrap()
}

fn new_project_public_key() -> PublicKey {
    PublicKey::from_account_id("GA73S4WXZG7EONFCIFDSZ6VOJKFC2PMV5574YDJC4V4UBDGPAYN4SPAC").unwrap()
}

fn new_credit_asset() -> Asset {
    let issuer =
        PublicKey::from_account_id("GB2O5PBQJDAFCNM2U2DIMVAEI7ISOYL4UJDTLN42JYYXAENKBWY6OBKZ")
            .unwrap();
    Asset::new_credit("USD", issuer).unwrap()
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

#[tokio::test]
async fn test_all_assets() {
    let client = new_client();
    let req = api::assets::all().with_asset_code("BTC");
    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_order_book() {
    let client = new_client();

    let req = api::aggregations::order_book(Asset::new_native(), new_credit_asset());
    let response = client.request(req).await.unwrap();
    assert_eq!("native", response.base.asset_type);
    assert_eq!("credit_alphanum4", response.counter.asset_type);
}

#[tokio::test]
async fn test_stream_order_book() {
    let client = new_client();
    let req = api::aggregations::order_book(Asset::new_native(), new_credit_asset());
    let mut stream = client
        .stream(req)
        .unwrap()
        .timeout(Duration::from_secs(1))
        .take(5);
    // break out of loop on timeout.
    loop {
        match stream.try_next().await {
            Err(_) => break,
            Ok(_) => {}
        }
    }
}

#[tokio::test]
async fn test_paths_strict_receive() {
    let client = new_client();
    let req = api::aggregations::paths_strict_receive(
        vec![],
        Asset::new_native(),
        Amount::from_str("0.3").unwrap(),
    )
    .unwrap()
    .with_source_account(&new_project_public_key());
    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_paths_strict_send() {
    let client = new_client();
    let req = api::aggregations::paths_strict_send(
        Asset::new_native(),
        vec![],
        Amount::from_str("0.3").unwrap(),
    )
    .unwrap()
    .with_destination_account(&new_project_public_key());
    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_all_trades_aggregation() {
    let client = new_client();
    let now = Utc::now();
    let start_time = now - ChronoDuration::weeks(4);

    let req = api::aggregations::all_trades(
        start_time,
        now,
        Resolution::OneDay,
        Asset::new_native(),
        new_credit_asset(),
    );
    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_fee_stats() {
    let client = new_client();
    let req = api::aggregations::fee_stats();
    let response = client.request(req).await.unwrap();
    let base_fee = response.last_ledger_base_fee.parse::<u64>().unwrap();
    assert!(base_fee >= 100);
}

#[tokio::test]
async fn test_all_offers() {
    let client = new_client();

    let req = api::offers::all()
        .with_limit(10)
        .with_selling(Asset::new_native())
        .with_buying(new_credit_asset());

    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_offers_for_account() {
    let client = new_client();

    let req = api::offers::for_account(&new_project_public_key())
        .with_cursor("now")
        .with_limit(10);

    let response = client.request(req).await.unwrap();
    assert!(response.records.is_empty());
}

#[tokio::test]
async fn test_single_offer() {
    let client = new_client();

    let req = api::offers::all()
        .with_limit(1)
        .with_selling(Asset::new_native())
        .with_buying(new_credit_asset());

    let response = client.request(req).await.unwrap();
    let offer = response.records.iter().next().unwrap();
    let offer_id = offer.id.parse::<i64>().unwrap();

    let req = api::offers::single(offer_id);
    let response = client.request(req).await.unwrap();
    assert_eq!(offer.id, response.id);
}

#[tokio::test]
async fn test_submit_transaction() {
    let client = new_client();
    let key_pair = new_project_key_pair();

    let account_req = api::accounts::single(key_pair.public_key());
    let account = client.request(account_req).await.unwrap();
    let sequence = account.sequence.parse::<i64>().unwrap();

    let data_value = DataValue::from_slice("Hello".as_bytes()).unwrap();
    let time_bounds = TimeBounds::valid_for(ChronoDuration::minutes(5));
    let mut tx = Transaction::builder(key_pair.public_key().clone(), sequence + 1, MIN_BASE_FEE)
        .with_time_bounds(time_bounds)
        .add_operation(
            Operation::new_manage_data()
                .with_data_name("Test".to_string())
                .with_data_value(Some(data_value))
                .build()
                .unwrap(),
        )
        .into_transaction()
        .unwrap()
        .into_envelope();

    tx.sign(&key_pair, &Network::new_public()).unwrap();

    let response = client
        .request(api::transactions::submit(&tx).unwrap())
        .await
        .unwrap();
    assert!(response.valid_before.is_some());
}

#[tokio::test]
async fn test_all_operations() {
    let client = new_client();

    let req = api::operations::all().with_join(api::operations::Join::Transactions);
    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_all_operations() {
    let client = new_client();

    let req = api::operations::all().with_join(api::operations::Join::Transactions);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_single_operation() {
    let client = new_client();

    let req = api::operations::all()
        .with_join(api::operations::Join::Transactions)
        .with_limit(1);
    let response = client.request(req).await.unwrap();
    let response_id = &response.records.iter().next().unwrap().base().id;

    let response = client
        .request(
            api::operations::single(response_id.to_string())
                .with_join(api::operations::Join::Transactions),
        )
        .await
        .unwrap();
    assert_eq!(&response.base().id, response_id);
}

#[tokio::test]
async fn test_operations_for_account() {
    let client = new_client();

    let req = api::operations::for_account(&new_project_public_key());
    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_operations_for_account() {
    let client = new_client();

    let req = api::operations::for_account(&new_project_public_key());
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_operations_for_ledger() {
    let client = new_client();
    let root = client.request(api::root::root()).await.unwrap();

    let req = api::operations::for_ledger(root.history_latest_ledger);
    let response = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_operations_for_ledger() {
    let client = new_client();
    let root = client.request(api::root::root()).await.unwrap();

    let req = api::operations::for_ledger(root.history_latest_ledger);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}
