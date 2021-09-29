use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.huobi.pro";

/// Huobi Spot market.
///
/// * REST API doc: <https://huobiapi.github.io/docs/spot/v1/en/>
/// * Trading at: <https://www.huobi.com/en-us/exchange/>
/// * Rate Limits: <https://huobiapi.github.io/docs/spot/v1/en/#rate-limiting-rule>
///   * If API Key is empty in request, then each IP is limited to 10 times per second
pub struct HuobiSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiSpotRestClient);

impl HuobiSpotRestClient {
    /// Get the latest Level2 orderbook snapshot.
    ///
    /// Top 150 bids and asks (aggregated) are returned.
    ///
    /// For example: <https://api.huobi.pro/market/depth?symbol=btcusdt&type=step0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/market/depth?symbol={}&type=step0", symbol))
    }
}
