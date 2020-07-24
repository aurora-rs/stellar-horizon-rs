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
impl_serde_test!(test_assets_serde, Page<AssetStat>, "./fixtures/assets.json");
impl_serde_test!(
    test_book_summary_serde,
    OrderBookSummary,
    "./fixtures/book_summary.json"
);
impl_serde_test!(test_ledger_serde, Ledger, "./fixtures/ledger.json");
impl_serde_test!(test_fee_stats_serde, FeeStats, "./fixtures/fee_stats.json");
impl_serde_test!(test_offers_serde, Page<Offer>, "./fixtures/all_offers.json");
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
