use std::sync::{Arc, Mutex};

use MarketDataFeed::binance::Binance;
use MarketDataFeed::common::ExchangeFeed;
use MarketDataFeed::common::create_exchange;
use MarketDataFeed::okx::OKX;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    let config_path = "configs/config_test.toml".to_string();
    let config = &config_path;

    start_binance(config);
    start_okx(config);

    let _ = ctrl_c().await;
    println!("Received Ctrl+C, shutting down...");
}

fn start_exchange(exchange: impl ExchangeFeed + Send + Sync + 'static) {
    let exchange_arc = Arc::new(Mutex::new(exchange));
    if !exchange_arc.lock().unwrap().enable() {
        println!("{} disabled.", exchange_arc.lock().unwrap().name());
        return;
    }

    exchange_arc.lock().unwrap().start();
    ExchangeFeed::connect_on_thread(exchange_arc);
}

fn start_binance(config_path: &String) {
    let binance: Binance = create_exchange(&config_path);
    start_exchange(binance);
}

fn start_okx(config_path: &String) {
    let okx: OKX = create_exchange(&config_path);
    start_exchange(okx);
}
