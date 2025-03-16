use std::sync::{Arc, Mutex};

use MarketDataFeed::binance::Binance;
use MarketDataFeed::common::ExchangeFeed;
use MarketDataFeed::common::create_exchange;
use MarketDataFeed::okx::OKX;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO) // Ensure INFO logs are enabled
        .init();

    tracing::info!("Starting Server.");

    let config_path = "configs/config.toml".to_string();
    let config = &config_path;

    start_binance(config);
    start_okx(config);

    let _ = ctrl_c().await;
    println!("Received Ctrl+C, shutting down...");
}

fn start_exchange(exchange: impl ExchangeFeed + Send + Sync + 'static, pin_id: usize) {
    let exchange_arc = Arc::new(Mutex::new(exchange));
    if !exchange_arc.lock().unwrap().enable() {
        println!("{} disabled.", exchange_arc.lock().unwrap().name());
        return;
    }

    exchange_arc.lock().unwrap().start();
    ExchangeFeed::connect_on_thread(exchange_arc, pin_id);
}

fn start_binance(config_path: &String) {
    let binance: Binance = create_exchange(&config_path);
    start_exchange(binance, 0);
}

fn start_okx(config_path: &String) {
    let okx: OKX = create_exchange(&config_path);
    start_exchange(okx,1);
}
