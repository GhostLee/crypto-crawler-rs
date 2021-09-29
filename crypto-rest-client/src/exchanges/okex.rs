use super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://www.okex.com/api";

/// The REST client for OKEx.
///
/// OKEx has Spot, Future, Swap and Option markets.
///
/// * WebSocket API doc: <https://www.okex.com/docs/en/>
/// * Trading at:
///     * Spot <https://www.bitmex.com/app/trade/>
///     * Future <https://www.okex.com/derivatives/futures>
///     * Swap <https://www.okex.com/derivatives/swap>
///     * Option <https://www.okex.com/derivatives/options>
pub struct OkexRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl OkexRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        OkexRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get most recent trades.
    ///
    /// 100 trades are returned.
    ///
    /// For example: <https://www.okex.com/api/spot/v3/instruments/BTC-USDT/trades>
    pub fn fetch_trades(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/{}/v3/instruments/{}/trades",
            pair_to_market_type(symbol),
            symbol
        ))
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 200 bids and asks are returned.
    ///
    /// For example:
    /// * <https://www.okex.com/api/spot/v3/instruments/BTC-USDT/book?size=200>,
    /// * <https://www.okex.com/api/swap/v3/instruments/BTC-USDT-SWAP/depth?size=200>
    ///
    /// Rate limit: 20 requests per 2 seconds
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/{}/v3/instruments/{}/{}?size=200",
            pair_to_market_type(symbol),
            symbol,
            if symbol.ends_with("-SWAP") {
                "depth"
            } else {
                "book"
            }
        ))
    }

    /// Get option underlying.
    pub fn fetch_option_underlying() -> Result<Vec<String>> {
        let txt = http_get(
            "https://www.okex.com/api/option/v3/underlying",
            &BTreeMap::new(),
        )?;
        let underlying_indexes = serde_json::from_str::<Vec<String>>(&txt)?;
        Ok(underlying_indexes)
    }
}

fn pair_to_market_type(pair: &str) -> &'static str {
    if pair.ends_with("-SWAP") {
        "swap"
    } else {
        let c = pair.matches('-').count();
        if c == 1 {
            "spot"
        } else if c == 2 {
            let date = &pair[(pair.len() - 6)..];
            debug_assert!(date.parse::<i64>().is_ok());
            "futures"
        } else {
            debug_assert!(pair.ends_with("-C") || pair.ends_with("-P"));
            "option"
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pair_to_market_type() {
        assert_eq!("spot", super::pair_to_market_type("BTC-USDT"));
        assert_eq!("futures", super::pair_to_market_type("BTC-USDT-210625"));
        assert_eq!("swap", super::pair_to_market_type("BTC-USDT-SWAP"));
        assert_eq!(
            "option",
            super::pair_to_market_type("BTC-USD-210625-72000-C")
        );
    }
}
