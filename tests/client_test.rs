use chrono::{Duration as ChronoDuration, Utc};
use std::env;
use std::str::FromStr;
use stellar_base::account::DataValue;
use stellar_base::amount::Amount;
use stellar_base::crypto::SodiumKeyPair;
use stellar_base::time_bounds::TimeBounds;
use stellar_base::transaction::MIN_BASE_FEE;
use stellar_base::{Asset, Network, Operation, PublicKey, Transaction};
use stellar_horizon::api;
use stellar_horizon::api::aggregations::Resolution;
use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
use stellar_horizon::headers::{rate_limit_limit, rate_limit_remaining, rate_limit_reset};
use stellar_horizon::request::{Order, PageRequest};
use tokio_stream::StreamExt;

fn new_client() -> HorizonHttpClient {
    HorizonHttpClient::new_from_str("https://horizon.stellar.org").unwrap()
}

fn new_client_public_node() -> HorizonHttpClient {
    HorizonHttpClient::new_from_str("https://horizon.publicnode.org").unwrap()
}

fn new_root_key() -> SodiumKeyPair {
    SodiumKeyPair::from_network(&Network::new_public()).unwrap()
}

fn new_project_key_pair() -> SodiumKeyPair {
    let secret_seed = env::var("SECRET_SEED").unwrap();
    SodiumKeyPair::from_secret_seed(&secret_seed).unwrap()
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
    let (_, response) = client.request(api::root::root()).await.unwrap();
    assert!(!response.horizon_version.is_empty());
}

#[tokio::test]
async fn test_headers() {
    // Use publicnode.org instead of stellar.org, since the latter doesn't have
    // rate limit fields (see
    // https://stellar.stackexchange.com/questions/6123/cant-find-horizon-x-ratelimit-headers)
    let client = new_client_public_node();
    let (headers, _) = client.request(api::root::root()).await.unwrap();
    let limit = rate_limit_limit(&headers).unwrap();
    let remaining = rate_limit_remaining(&headers).unwrap();
    let reset = rate_limit_reset(&headers).unwrap();
    assert!(limit > 0);
    assert!(remaining > 0);
    assert!(reset > 0);
}

#[tokio::test]
async fn test_single_ledger() {
    let client = new_client();
    let (_, root) = client.request(api::root::root()).await.unwrap();
    let (_, response) = client
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
    let (_, response) = client.request(req).await.unwrap();
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
    let root_key = new_root_key().public_key();
    let req = api::accounts::single(&root_key);
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(root_key.account_id(), response.paging_token);
}

#[tokio::test]
async fn test_all_accounts_by_signer() {
    let client = new_client();
    let key = new_project_public_key();
    let req = api::accounts::all().with_signer(&key);
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(1, response.records.len());
}

#[tokio::test]
async fn test_all_accounts_by_asset() {
    let client = new_client();
    let asset = new_credit_asset().as_credit().unwrap().clone();
    let req = api::accounts::all()
        .with_trusted_asset(asset)
        .with_limit(30);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_all_transactions() {
    let client = new_client();
    let req = api::transactions::all()
        .with_order(&Order::Descending)
        .with_include_failed(true)
        .with_limit(5);
    let (_, response) = client.request(req).await.unwrap();
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
    let root_key = new_root_key().public_key();
    let req = api::transactions::for_account(&root_key)
        .with_order(&Order::Descending)
        .with_include_failed(true)
        .with_limit(5);
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(response.records.len(), 5);
}

#[tokio::test]
async fn test_stream_transactions_for_account() {
    let client = new_client();
    let root_key = new_root_key().public_key();
    let req = api::transactions::for_account(&root_key).with_include_failed(true);
    let mut stream = client.stream(req).unwrap().take(2);
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().paging_token.is_empty());
    }
}

#[tokio::test]
async fn test_transactions_for_ledger() {
    let client = new_client();
    let (_, root) = client.request(api::root::root()).await.unwrap();
    let req = api::transactions::for_ledger(root.history_latest_ledger);
    let _response = client.request(req).await.unwrap();
}

#[tokio::test]
async fn test_stream_transactions_for_ledger() {
    let client = new_client();
    let (_, root) = client.request(api::root::root()).await.unwrap();
    let req = api::transactions::for_ledger(root.history_latest_ledger);
    let mut stream = client.stream(req).unwrap().take(1);
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().paging_token.is_empty());
    }
}

// Ignore test as horizon tends to fail on this one
#[ignore]
#[tokio::test]
async fn test_transactions_for_claimable_balance() {
    let claimable_balance_id =
        "00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9";
    let client = new_client();
    let req = api::transactions::for_claimable_balance(claimable_balance_id)
        .with_order(&Order::Descending)
        .with_include_failed(true)
        .with_limit(5);
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(response.records.len(), 1);
}

// Ignore test as horizon tends to fail on this one
#[ignore]
#[tokio::test]
async fn test_stream_transactions_for_claimable_balance() {
    let claimable_balance_id =
        "00000000178826fbfe339e1f5c53417c6fedfe2c05e8bec14303143ec46b38981b09c3f9";
    let client = new_client();
    let req =
        api::transactions::for_claimable_balance(claimable_balance_id).with_include_failed(true);
    let mut stream = client.stream(req).unwrap().take(1);
    let mut count = 0;
    while let Some(_) = stream.next().await {
        count += 1;
    }
    assert_eq!(1, count);
}

#[tokio::test]
async fn test_transactions_for_liquidity_pool() {
    let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";

    let client = new_client();

    let req = api::transactions::for_liquidity_pool(liquidity_pool_id);
    let (_, response) = client.request(req).await.unwrap();

    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_transactions_for_liquidity_pool() {
    let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";

    let client = new_client();

    let req = api::transactions::for_liquidity_pool(liquidity_pool_id);
    let mut stream = client.stream(req).unwrap().take(1);
    while let Some(event) = stream.next().await {
        assert!(!event.unwrap().id.is_empty());
    }
}

#[tokio::test]
async fn test_all_trades() {
    let client = new_client();
    let req = api::trades::all()
        .with_order(&Order::Descending)
        .with_limit(5);
    let (_, response) = client.request(req).await.unwrap();
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
    let root_key = new_root_key().public_key();
    let req = api::trades::for_account(&root_key)
        .with_order(&Order::Descending)
        .with_limit(5);
    let (_, response) = client.request(req).await.unwrap();
    assert!(response.records.is_empty());
}

#[tokio::test]
async fn test_trades_for_liquidity_pool() {
    let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";

    let client = new_client();

    let req = api::trades::for_liquidity_pool(liquidity_pool_id);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_trades_for_liquidity_pool() {
    let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";

    let client = new_client();

    let req = api::trades::for_liquidity_pool(liquidity_pool_id);

    let mut stream = client.stream(req).unwrap().take(1);
    let mut count = 0;
    while let Some(_event) = stream.next().await {
        count += 1;
    }
    assert_eq!(1, count);
}

#[tokio::test]
async fn test_data_for_account() {
    let client = new_client();
    let root_key = new_root_key().public_key();
    let req = api::data::for_account(&root_key, "FooBar");
    let response = client.request(req).await;
    assert!(response.is_err());
}

#[tokio::test]
async fn test_all_assets() {
    let client = new_client();
    let req = api::assets::all().with_asset_code("BTC");
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_all_assets_with_code_alphanum12() {
    let client = new_client();
    let req = api::assets::all().with_asset_code("LEVELG");
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_order_book() {
    let client = new_client();

    let req = api::aggregations::order_book(Asset::new_native(), new_credit_asset());
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!("native", response.base.asset_type);
    assert_eq!("credit_alphanum4", response.counter.asset_type);
}

/*
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
*/

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
    let (_, response) = client.request(req).await.unwrap();
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
    let (_, response) = client.request(req).await.unwrap();
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
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_fee_stats() {
    let client = new_client();
    let req = api::aggregations::fee_stats();
    let (_, response) = client.request(req).await.unwrap();
    assert!(response.last_ledger_base_fee >= 100);
}

#[tokio::test]
async fn test_all_offers() {
    let client = new_client();

    let req = api::offers::all()
        .with_limit(10)
        .with_selling(Asset::new_native())
        .with_buying(new_credit_asset());

    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_offers_for_account() {
    let client = new_client();

    let req = api::offers::for_account(&new_project_public_key())
        .with_cursor("now")
        .with_limit(10);

    let (_, response) = client.request(req).await.unwrap();
    assert!(response.records.is_empty());
}

#[tokio::test]
async fn test_single_offer() {
    let client = new_client();

    let req = api::offers::all()
        .with_limit(1)
        .with_selling(Asset::new_native())
        .with_buying(new_credit_asset());

    let (_, response) = client.request(req).await.unwrap();
    let offer = response.records.first().unwrap();

    let req = api::offers::single(offer.id);
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(offer.id, response.id);
}

#[tokio::test]
async fn test_submit_transaction() {
    let client = new_client();
    let key_pair = new_project_key_pair();

    let account_req = api::accounts::single(&key_pair.public_key());
    let (_, account) = client.request(account_req).await.unwrap();
    let sequence = account.sequence.parse::<i64>().unwrap();

    let data_value = DataValue::from_slice("Hello".as_bytes()).unwrap();
    let time_bounds = TimeBounds::valid_for(ChronoDuration::minutes(5));
    let mut tx = Transaction::builder(key_pair.public_key(), sequence + 1, MIN_BASE_FEE)
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

    tx.sign(key_pair.as_ref(), &Network::new_public()).unwrap();

    let (_, response) = client
        .request(api::transactions::submit(&tx).unwrap())
        .await
        .unwrap();
    assert!(response.valid_before.is_some());
}

#[tokio::test]
async fn test_all_claimable_balances() {
    let client = new_client();

    let req = api::claimable_balances::all();
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_single_claimable_balance() {
    let client = new_client();

    let req = api::claimable_balances::all();
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());

    let claimable_balance_id = response.records.first().unwrap().id.clone();
    let req = api::claimable_balances::single(claimable_balance_id.clone());
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(claimable_balance_id, response.id);
}

#[tokio::test]
async fn test_all_operations() {
    let client = new_client();

    let req = api::operations::all().with_join(api::Join::Transactions);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_all_operations() {
    let client = new_client();

    let req = api::operations::all().with_join(api::Join::Transactions);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_single_operation() {
    let client = new_client();

    let req = api::operations::all()
        .with_join(api::Join::Transactions)
        .with_limit(1);
    let (_, response) = client.request(req).await.unwrap();
    let response_id = &response.records.first().unwrap().base().id;

    let (_, response) = client
        .request(
            api::operations::single(response_id.to_string()).with_join(api::Join::Transactions),
        )
        .await
        .unwrap();
    assert_eq!(&response.base().id, response_id);
}

#[tokio::test]
async fn test_operations_for_account() {
    let client = new_client();

    let req = api::operations::for_account(&new_project_public_key());
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_operations_for_account() {
    let client = new_client();

    let req = api::operations::for_account(&new_project_public_key());
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_operations_for_ledger() {
    let client = new_client();
    let (_, root) = client.request(api::root::root()).await.unwrap();

    let req = api::operations::for_ledger(root.history_latest_ledger);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_operations_for_ledger() {
    let client = new_client();
    let (_, root) = client.request(api::root::root()).await.unwrap();

    let req = api::operations::for_ledger(root.history_latest_ledger);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

// Ignore test as horizon tends to fail on this one
#[ignore]
#[tokio::test]
async fn test_operations_for_claimable_balance() {
    let claimable_balance_id =
        "00000000ffec74e0fe7d94bdb1b257dbeef1695cbf3611f538908357a5ee57924edc452f";
    let client = new_client();

    let req = api::operations::for_claimbable_balance(claimable_balance_id);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

// Ignore test as horizon tends to fail on this one
#[ignore]
#[tokio::test]
async fn test_stream_operations_for_claimable_balance() {
    let claimable_balance_id =
        "00000000ffec74e0fe7d94bdb1b257dbeef1695cbf3611f538908357a5ee57924edc452f";
    let client = new_client();

    let req = api::operations::for_claimbable_balance(claimable_balance_id);
    let mut stream = client.stream(req).unwrap().take(1);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(1, count);
}

#[tokio::test]
async fn test_operations_for_liquidity_pool() {
    let liquidity_pool_id = "6d30e1f5721962d8bad07d90c606a3963ddbe23c8751cdbdc87224d188f4593c";
    let client = new_client();

    let req = api::operations::for_liquidity_pool(liquidity_pool_id);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_operations_for_liquidity_pool() {
    let liquidity_pool_id = "6d30e1f5721962d8bad07d90c606a3963ddbe23c8751cdbdc87224d188f4593c";
    let client = new_client();

    let req = api::operations::for_liquidity_pool(liquidity_pool_id);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_all_payments() {
    let client = new_client();

    let req = api::payments::all().with_join(api::Join::Transactions);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_all_payments() {
    let client = new_client();

    let req = api::payments::all().with_join(api::Join::Transactions);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_payments_for_account() {
    let client = new_client();

    let req = api::payments::for_account(&new_project_public_key());
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_payments_for_account() {
    let client = new_client();

    let req = api::payments::for_account(&new_project_public_key());
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_all_effects() {
    let client = new_client();

    let req = api::effects::all();
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_all_effects() {
    let client = new_client();

    let req = api::effects::all();
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_effects_for_transaction() {
    let client = new_client();

    let tx_req = api::transactions::all().with_limit(1);
    let (_, tx_response) = client.request(tx_req).await.unwrap();
    let tx_hash = &tx_response.records.first().unwrap().id;

    let req = api::effects::for_transaction(tx_hash.to_string());
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_effects_for_operation() {
    let client = new_client();

    let op_req = api::operations::all().with_limit(1);
    let (_, op_response) = client.request(op_req).await.unwrap();
    let op_id = &op_response.records.first().unwrap().base().id;

    let req = api::effects::for_operation(op_id.to_string());
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_effects_for_ledger() {
    let client = new_client();
    let (_, root) = client.request(api::root::root()).await.unwrap();

    let req = api::effects::for_ledger(root.history_latest_ledger);
    let _response = client.request(req).await.unwrap();
    // records can be empty
}

#[tokio::test]
async fn test_stream_effects_for_ledger() {
    let client = new_client();
    let (_, root) = client.request(api::root::root()).await.unwrap();

    let req = api::effects::for_ledger(root.history_latest_ledger);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_effects_for_account() {
    let client = new_client();

    let req = api::effects::for_account(&new_project_public_key());
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_effects_for_account() {
    let client = new_client();

    let req = api::effects::for_account(&new_project_public_key());
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_effects_for_liquidity_pool() {
    let client = new_client();

    let liquidity_pool_id = "006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207";

    let req = api::effects::for_liquidity_pool(liquidity_pool_id);
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_stream_effects_for_liquidity_pool() {
    let client = new_client();

    let liquidity_pool_id = "006881bb9a17b0c0f4000cb12eaeb2b954390707b03a676b87f824dc6af9f207";

    let req = api::effects::for_liquidity_pool(liquidity_pool_id);
    let mut stream = client.stream(req).unwrap().take(3);
    let mut count = 0;
    while let Some(_event) = stream.try_next().await.unwrap() {
        count += 1;
    }
    assert_eq!(3, count);
}

#[tokio::test]
async fn test_all_liquidity_pools() {
    let client = new_client();

    let req = api::liquidity_pools::all();
    let (_, response) = client.request(req).await.unwrap();
    assert!(!response.records.is_empty());
}

#[tokio::test]
async fn test_single_liquidity_pool() {
    let client = new_client();

    let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";

    let req = api::liquidity_pools::single(liquidity_pool_id.to_string());
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(liquidity_pool_id, response.id);
}

#[tokio::test]
async fn test_extra_headers() {
    let mut headers = hyper::header::HeaderMap::new();
    headers.insert(
        "X-Client-Name",
        hyper::header::HeaderValue::from_static("test"),
    );
    let client =
        HorizonHttpClient::with_extra_headers("https://horizon.stellar.org", headers).unwrap();
    let liquidity_pool_id = "0016ed5f76feb9f407a3676be3c96448c44e61298e8e5ba0f23011350212fc16";

    let req = api::liquidity_pools::single(liquidity_pool_id.to_string());
    let (_, response) = client.request(req).await.unwrap();
    assert_eq!(liquidity_pool_id, response.id);
}

/*
#[tokio::test]
async fn test_pagination() {
    let client = new_client();
    let req = api::ledgers::all()
        .with_order(&Order::Descending)
        .with_cursor("now");
    let response = client.request(req).await.unwrap();
    let prev = response.previous.unwrap();
    let prev_response = client.request(prev).await.unwrap();
    let next = prev_response.next.unwrap();
    let _next_response = client.request(next).await.unwrap();
}

#[tokio::test]
async fn test_navigation_links() {
    let client = new_client();
    let req = api::ledgers::all()
        .with_order(&Order::Descending)
        .with_cursor("now");
    let response = client.request(req).await.unwrap();
    let ledger = response.records.iter().next().unwrap();
    let _tx_response = client
        .request(ledger.links.transactions.clone())
        .await
        .unwrap();
}
*/
