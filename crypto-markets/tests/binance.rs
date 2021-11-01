use crypto_markets::{fetch_markets, fetch_symbols, get_market_types, MarketType};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "binance";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert_eq!(symbol.to_uppercase(), symbol.to_string());
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = &symbol[(symbol.len() - 10)..(symbol.len() - 7)];
        assert_eq!(quote, "USD");
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = &symbol[(symbol.len() - 11)..(symbol.len() - 7)];
        assert_eq!(quote, "USDT");
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD_PERP"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USDT") || symbol.ends_with("BUSD"));
    }
}

#[test]
fn fetch_option_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-P") || symbol.ends_with("-C"));
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets.iter().find(|m| m.symbol == "BTCUSDT").unwrap();
    assert_eq!(btc_usdt.precision.price, 2);
    assert_eq!(btc_usdt.precision.quantity, 5);
    assert_eq!(btc_usdt.quantity_limit.min, 0.00001);
    assert_eq!(btc_usdt.quantity_limit.max, 9000.0);
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd_perp = markets.iter().find(|m| m.symbol == "BTCUSD_PERP").unwrap();
    assert_eq!(btcusd_perp.precision.price, 1);
    assert_eq!(btcusd_perp.precision.quantity, 0);
    assert_eq!(btcusd_perp.quantity_limit.min, 1.0);
    assert_eq!(btcusd_perp.quantity_limit.max, 1000000.0);
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());
}

#[test]
fn fetch_option_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!markets.is_empty());
}
