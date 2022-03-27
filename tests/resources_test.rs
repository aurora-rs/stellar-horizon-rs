use stellar_horizon::horizon_error::HorizonError;
use stellar_horizon::page::Page;
use stellar_horizon::resources::*;

macro_rules! impl_serde_test {
    ($test_name:ident, $name:path, $fixture:tt) => {
        #[test]
        fn $test_name() {
            let original_json_value = json::parse(include_str!($fixture)).unwrap();
            let original_json = original_json_value.dump();
            let account: $name = serde_json::from_str(&original_json).unwrap();
            let back_json = serde_json::to_string(&account).unwrap();
            let back: $name = serde_json::from_str(&back_json).unwrap();
            assert_eq!(account, back);
        }
    };
}

impl_serde_test!(test_account_serde, Account, "./fixtures/account.json");
impl_serde_test!(
    test_account_with_sponsor_serde,
    Account,
    "./fixtures/account_with_sponsor.json"
);
impl_serde_test!(
    test_account_data_serde,
    AccountData,
    "./fixtures/account_data.json"
);
impl_serde_test!(test_assets_serde, Page<AssetStat>, "./fixtures/assets.json");
impl_serde_test!(
    test_book_summary_serde,
    OrderBookSummary,
    "./fixtures/book_summary.json"
);
impl_serde_test!(
    test_claimable_balance,
    ClaimableBalance,
    "./fixtures/claimable_balance.json"
);
impl_serde_test!(
    test_claimable_balances,
    Page<ClaimableBalance>,
    "./fixtures/all_claimable_balances.json"
);
impl_serde_test!(test_ledger_serde, Ledger, "./fixtures/ledger.json");
impl_serde_test!(test_fee_stats_serde, FeeStats, "./fixtures/fee_stats.json");
impl_serde_test!(test_offers_serde, Page<Offer>, "./fixtures/all_offers.json");
impl_serde_test!(
    test_offers_with_sponsor_serde,
    Page<Offer>,
    "./fixtures/all_offers_with_sponsor.json"
);
impl_serde_test!(test_root_serde, Root, "./fixtures/root.json");
impl_serde_test!(test_trades_serde, Page<Trade>, "./fixtures/all_trades.json");
impl_serde_test!(
    test_paths_strict_receive_serde,
    Page<Path>,
    "./fixtures/paths_strict_receive.json"
);
impl_serde_test!(
    test_paths_strict_send_serde,
    Page<Path>,
    "./fixtures/paths_strict_send.json"
);
impl_serde_test!(
    test_trade_aggregations_serde,
    Page<TradeAggregation>,
    "./fixtures/all_trade_aggregations.json"
);
impl_serde_test!(
    test_effects_serde,
    Page<Effect>,
    "./fixtures/all_effects.json"
);
impl_serde_test!(
    test_operations_serde,
    Page<Operation>,
    "./fixtures/all_operations.json"
);
impl_serde_test!(
    test_payments_serde,
    Page<Payment>,
    "./fixtures/all_payments.json"
);
impl_serde_test!(
    test_transactions_serde,
    Page<Transaction>,
    "./fixtures/all_transactions.json"
);
impl_serde_test!(
    test_liquidity_pools_serde,
    Page<LiquidityPool>,
    "./fixtures/all_liquidity_pools.json"
);
impl_serde_test!(
    test_single_liquidity_pool_serde,
    LiquidityPool,
    "./fixtures/liquidity_pool.json"
);

#[test]
fn test_operations_base() {
    let original_json_value = json::parse(include_str!("./fixtures/all_operations.json")).unwrap();
    let original_json = original_json_value.dump();
    let operations: Page<Operation> = serde_json::from_str(&original_json).unwrap();
    for operation in operations.records {
        assert!(!operation.base().paging_token.is_empty());
    }
}

#[test]
fn test_effects_base() {
    let original_json_value = json::parse(include_str!("./fixtures/all_effects.json")).unwrap();
    let original_json = original_json_value.dump();
    let effects: Page<Effect> = serde_json::from_str(&original_json).unwrap();
    for effect in effects.records {
        assert!(!effect.base().paging_token.is_empty());
    }
}

impl_serde_test!(
    test_horizon_error_bad_request,
    HorizonError,
    "./fixtures/error_bad_request.json"
);

impl_serde_test!(
    test_horizon_error_transaction_failed,
    HorizonError,
    "./fixtures/error_transaction_failed.json"
);

impl_serde_test!(
    test_horizon_error_transaction_malformed,
    HorizonError,
    "./fixtures/error_transaction_malformed.json"
);

impl_serde_test!(
    test_horizon_error_before_history,
    HorizonError,
    "./fixtures/error_before_history.json"
);

impl_serde_test!(
    test_horizon_error_stale_history,
    HorizonError,
    "./fixtures/error_stale_history.json"
);

impl_serde_test!(
    test_horizon_error_timeout,
    HorizonError,
    "./fixtures/error_timeout.json"
);

impl_serde_test!(
    test_horizon_error_invalid_accounts_params,
    HorizonError,
    "./fixtures/error_invalid_accounts_params.json"
);

impl_serde_test!(
    test_horizon_error_invalid_order_book,
    HorizonError,
    "./fixtures/error_invalid_order_book.json"
);
