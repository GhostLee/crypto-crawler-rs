use crate::WSClient;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use super::{
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "binance";

pub(super) const WEBSOCKET_URL: &str = "wss://stream.opsnest.com/stream";

const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (240, r#"{"event":"ping"}"#);

/// Binance Option market
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/voptions/en/>
///   * Trading at: <https://voptions.binance.com/en>
pub struct BinanceOptionWSClient {
    client: WSClientInternal,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let raw_channels: Vec<&String> = channels.iter().filter(|ch| !ch.starts_with('{')).collect();
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    if !raw_channels.is_empty() {
        all_commands.append(&mut vec![format!(
            r#"{{"id":9527,"method":"{}","params":{}}}"#,
            if subscribe {
                "SUBSCRIBE"
            } else {
                "UNSUBSCRIBE"
            },
            serde_json::to_string(&raw_channels).unwrap()
        )])
    };

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == r#"{"id":9527}"# {
        return MiscMessage::Misc;
    } else if msg == r#"{"event":"pong"}"# {
        return MiscMessage::Pong;
    }

    let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    if obj.contains_key("error") {
        error!("Received {} from {}", msg, EXCHANGE_NAME);
        panic!("Received {} from {}", msg, EXCHANGE_NAME);
    }

    if let Some(result) = obj.get("result") {
        if serde_json::Value::Null == *result {
            return MiscMessage::Misc;
        }
    }

    if !obj.contains_key("stream") || !obj.contains_key("data") {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }

    MiscMessage::Normal
}

impl_new_constructor!(
    BinanceOptionWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(BinanceOptionWSClient);

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}@{}", pair, channel)
}

#[rustfmt::skip]
impl_trait!(Trade, BinanceOptionWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BinanceOptionWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, BinanceOptionWSClient, subscribe_bbo, "bookTicker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BinanceOptionWSClient, subscribe_orderbook, "depth@100ms", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BinanceOptionWSClient, subscribe_orderbook_topk, "depth10", to_raw_channel);

fn to_candlestick_raw_channel(symbol: &str, interval: usize) -> String {
    let interval_str = match interval {
        60 => "1m",
        300 => "5m",
        900 => "15m",
        1800 => "30m",
        3600 => "1h",
        14400 => "4h",
        86400 => "1d",
        604800 => "1w",
        _ => panic!("Binance has intervals 1m,5m,15m,30m,1h4h,1d,1w"),
    };
    format!("{}@kline_{}", symbol, interval_str)
}

impl_candlestick!(BinanceOptionWSClient);

panic_l3_orderbook!(BinanceOptionWSClient);
