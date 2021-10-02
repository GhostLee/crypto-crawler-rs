use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::time::Duration;

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "huobi";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, HuobiSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_future, HuobiFutureWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_swap, HuobiLinearSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_swap, HuobiInverseSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_option, HuobiOptionWSClient, MessageType::Trade, subscribe_trade);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_future, HuobiFutureWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_swap, HuobiLinearSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_swap, HuobiInverseSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_option, HuobiOptionWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_spot, HuobiSpotWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_inverse_future, HuobiFutureWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_linear_swap, HuobiLinearSwapWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_inverse_swap, HuobiInverseSwapWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_option, HuobiOptionWSClient, MessageType::BBO, subscribe_bbo);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_spot, HuobiSpotWSClient, MessageType::L2TopK, subscribe_orderbook_topk);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_inverse_future, HuobiFutureWSClient, MessageType::L2TopK, subscribe_orderbook_topk);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_linear_swap, HuobiLinearSwapWSClient, MessageType::L2TopK, subscribe_orderbook_topk);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_inverse_swap, HuobiInverseSwapWSClient, MessageType::L2TopK, subscribe_orderbook_topk);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_option, HuobiOptionWSClient, MessageType::L2TopK, subscribe_orderbook_topk);

#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_spot, HuobiSpotWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_inverse_future, HuobiFutureWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_linear_swap, HuobiLinearSwapWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_inverse_swap, HuobiInverseSwapWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_option, HuobiOptionWSClient, MessageType::Ticker, subscribe_ticker);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseFuture => {
            crawl_trade_inverse_future(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => crawl_trade_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => crawl_trade_inverse_swap(market_type, symbols, on_msg, duration),
        MarketType::EuropeanOption => crawl_trade_option(market_type, symbols, on_msg, duration),
        _ => panic!("Huobi does NOT have the {} market type", market_type),
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => {
            let on_msg_ext = |msg: String| {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    market_type,
                    MessageType::L2Event,
                    msg,
                );
                (on_msg.lock().unwrap())(message);
            };
            let symbols: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
                fetch_symbols_retry(EXCHANGE_NAME, market_type)
            } else {
                symbols.unwrap().to_vec()
            };
            // Huobi Spot market.$symbol.mbp.$levels must use wss://api.huobi.pro/feed
            // or wss://api-aws.huobi.pro/feed
            let ws_client = HuobiSpotWSClient::new(
                Arc::new(Mutex::new(on_msg_ext)),
                Some("wss://api.huobi.pro/feed"),
            );
            ws_client.subscribe_orderbook(&symbols);
            ws_client.run(duration);
            None
        }
        MarketType::InverseFuture => {
            crawl_l2_event_inverse_future(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => {
            crawl_l2_event_linear_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::InverseSwap => {
            crawl_l2_event_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::EuropeanOption => crawl_l2_event_option(market_type, symbols, on_msg, duration),
        _ => panic!("Huobi does NOT have the {} market type", market_type),
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
        MarketType::InverseFuture => {
            crawl_bbo_inverse_future(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => crawl_bbo_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => crawl_bbo_inverse_swap(market_type, symbols, on_msg, duration),
        MarketType::EuropeanOption => crawl_bbo_option(market_type, symbols, on_msg, duration),
        _ => panic!("Huobi does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_topk(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_l2_topk_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseFuture => {
            crawl_l2_topk_inverse_future(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => crawl_l2_topk_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => {
            crawl_l2_topk_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::EuropeanOption => crawl_l2_topk_option(market_type, symbols, on_msg, duration),
        _ => panic!("Huobi does NOT have the {} market type", market_type),
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
        MarketType::InverseFuture => {
            crawl_ticker_inverse_future(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => crawl_ticker_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => {
            crawl_ticker_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::EuropeanOption => crawl_ticker_option(market_type, symbols, on_msg, duration),
        _ => panic!("Huobi does NOT have the {} market type", market_type),
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
        let message = Message::new(
            EXCHANGE_NAME.to_string(),
            market_type,
            MessageType::FundingRate,
            msg,
        );
        (on_msg.lock().unwrap())(message);
    }));

    let symbols: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
        vec!["*".to_string()]
    } else {
        symbols.unwrap().to_vec()
    };
    let channels: Vec<String> = symbols
        .into_iter()
        .map(|symbol| format!(r#"{{"topic":"public.{}.funding_rate","op":"sub"}}"#, symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap => {
            let ws_client = HuobiInverseSwapWSClient::new(
                on_msg_ext,
                Some("wss://api.hbdm.com/swap-notification"),
            );
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        MarketType::LinearSwap => {
            let ws_client = HuobiLinearSwapWSClient::new(
                on_msg_ext,
                Some("wss://api.hbdm.com/linear-swap-notification"),
            );
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("Huobi {} does NOT have funding rates", market_type),
    }
}
