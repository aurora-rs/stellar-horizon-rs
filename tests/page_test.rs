use stellar_horizon::page::Page;
use stellar_horizon::resources::Ledger;

#[test]
fn test_parse_page_from_json() {
    let json = include_str!("./fixtures/all_ledgers.json");
    let page: Page<Ledger> = serde_json::from_str(json).unwrap();
    assert_eq!(3, page.records.len());
}

#[test]
fn test_serialize_page_to_json() {
    let original_json = include_str!("./fixtures/all_ledgers.json");
    let page: Page<Ledger> = serde_json::from_str(original_json).unwrap();
    let json = serde_json::to_string(&page).unwrap();
    let back: Page<Ledger> = serde_json::from_str(&json).unwrap();
    assert_eq!(page.records.len(), back.records.len());
    assert_eq!(
        page.links.as_ref().unwrap().self_,
        back.links.as_ref().unwrap().self_
    );
    assert_eq!(
        page.links.as_ref().unwrap().next,
        back.links.as_ref().unwrap().next
    );
    assert_eq!(
        page.links.as_ref().unwrap().previous,
        back.links.as_ref().unwrap().previous
    );
}
