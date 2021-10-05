use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use std::time::Duration;

use super::utils::{
    check_args, fetch_symbols_retry, get_candlestick_intervals, get_connection_interval_ms,
    get_send_interval_ms,
};
use crate::utils::WS_LOCKS;
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "gate";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, GateSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_swap, GateInverseSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_swap, GateLinearSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_future, GateLinearFutureWSClient, MessageType::Trade, subscribe_trade);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, GateSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_swap, GateInverseSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_swap, GateLinearSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_future, GateLinearFutureWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_spot, GateSpotWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_inverse_swap, GateInverseSwapWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_linear_swap, GateLinearSwapWSClient, MessageType::BBO, subscribe_bbo);

#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_spot, GateSpotWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_inverse_swap, GateInverseSwapWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_linear_swap, GateLinearSwapWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_linear_future, GateLinearFutureWSClient, MessageType::Ticker, subscribe_ticker);

#[rustfmt::skip]
gen_crawl_candlestick!(crawl_candlestick_spot, GateSpotWSClient);
#[rustfmt::skip]
gen_crawl_candlestick!(crawl_candlestick_inverse_swap, GateInverseSwapWSClient);
#[rustfmt::skip]
gen_crawl_candlestick!(crawl_candlestick_linear_swap, GateLinearSwapWSClient);
#[rustfmt::skip]
gen_crawl_candlestick!(crawl_candlestick_linear_future, GateLinearFutureWSClient);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => crawl_trade_inverse_swap(market_type, symbols, on_msg, duration),
        MarketType::LinearSwap => crawl_trade_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::LinearFuture => {
            crawl_trade_linear_future(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Gate does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_l2_event_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => {
            crawl_l2_event_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => {
            crawl_l2_event_linear_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearFuture => {
            crawl_l2_event_linear_future(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Gate does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_bbo_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => crawl_bbo_inverse_swap(market_type, symbols, on_msg, duration),
        MarketType::LinearSwap => crawl_bbo_linear_swap(market_type, symbols, on_msg, duration),
        _ => panic!("Gate does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_ticker_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => {
            crawl_ticker_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => crawl_ticker_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::LinearFuture => {
            crawl_ticker_linear_future(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Gate does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_candlestick(
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => {
            crawl_candlestick_spot(market_type, symbol_interval_list, on_msg, duration)
        }
        MarketType::InverseSwap => {
            crawl_candlestick_inverse_swap(market_type, symbol_interval_list, on_msg, duration)
        }
        MarketType::LinearSwap => {
            crawl_candlestick_linear_swap(market_type, symbol_interval_list, on_msg, duration)
        }
        MarketType::LinearFuture => {
            crawl_candlestick_linear_future(market_type, symbol_interval_list, on_msg, duration)
        }
        _ => panic!("Gate does NOT have the {} market type", market_type),
    }
}
