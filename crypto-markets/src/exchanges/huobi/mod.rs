mod utils;

pub(super) mod huobi_future;
pub(super) mod huobi_inverse_swap;
pub(super) mod huobi_linear_swap;
pub(super) mod huobi_option;
pub(super) mod huobi_spot;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => huobi_spot::fetch_spot_symbols(),
        MarketType::InverseFuture => huobi_future::fetch_inverse_future_symbols(),
        MarketType::InverseSwap => huobi_inverse_swap::fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => huobi_linear_swap::fetch_linear_swap_symbols(),
        MarketType::EuropeanOption => huobi_option::fetch_option_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => huobi_spot::fetch_spot_markets(),
        MarketType::InverseFuture => huobi_future::fetch_inverse_future_markets(),
        MarketType::InverseSwap => huobi_inverse_swap::fetch_inverse_swap_markets(),
        MarketType::LinearSwap => huobi_linear_swap::fetch_linear_swap_markets(),
        MarketType::EuropeanOption => Ok(Vec::new()),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}
