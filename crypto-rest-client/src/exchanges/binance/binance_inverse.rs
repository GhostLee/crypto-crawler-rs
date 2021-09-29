use super::super::utils::http_get;
use super::utils::*;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://dapi.binance.com";

/// Binance Coin-margined Future and Swap market
///
/// * REST API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
/// * Trading at: <https://www.binance.com/en/delivery/btcusd_perpetual>
/// Rate Limits: <https://binance-docs.github.io/apidocs/delivery/en/#limits>
///   * 2400 request weight per minute
pub struct BinanceInverseRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BinanceInverseRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BinanceInverseRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get compressed, aggregate trades.
    ///
    /// Equivalent to `/dapi/v1/aggTrades` with `limit=1000`
    ///
    /// For example:
    ///
    /// - <https://dapi.binance.com/dapi/v1/aggTrades?symbol=BTCUSD_PERP&limit=1000>
    /// - <https://dapi.binance.com/dapi/v1/aggTrades?symbol=BTCUSD_210625&limit=1000>
    pub fn fetch_agg_trades(
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<String> {
        check_symbol(symbol);
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api_binance!(
            "/dapi/v1/aggTrades",
            symbol,
            from_id,
            start_time,
            end_time,
            limit
        )
    }

    /// Get a Level2 snapshot of orderbook.
    ///
    /// Equivalent to `/dapi/v1/depth` with `limit=1000`
    ///
    /// For example:
    ///
    /// - <https://dapi.binance.com/dapi/v1/depth?symbol=BTCUSD_PERP&limit=1000>
    /// - <https://dapi.binance.com/dapi/v1/depth?symbol=BTCUSD_210625&limit=1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        check_symbol(symbol);
        let symbol = Some(symbol);
        let limit = Some(1000);
        gen_api_binance!("/dapi/v1/depth", symbol, limit)
    }
}
