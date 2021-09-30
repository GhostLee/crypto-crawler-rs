use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, KrakenRestClient};

#[test]
fn test_trades() {
    let text = KrakenRestClient::fetch_trades("XXBTZUSD", None).unwrap();
    assert!(text.starts_with("{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("kraken", MarketType::Spot, "XXBTZUSD", Some(3)).unwrap();
    assert!(text.starts_with("{"));
}
