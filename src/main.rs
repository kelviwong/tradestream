use std::thread;
use tokio::runtime::Builder;

use MarketDataFeed::binance::Binance;
use MarketDataFeed::okx::OKX;
use MarketDataFeed::common::ExchangeFeed;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    let success_callback = |message: String| {
        println!("Success callback received message: {}", message);
    };

    let symbols = "btcusdt@kline_1m";
    startBinance(symbols.to_string(), success_callback);

    // let symbols = "BTC-USDT@candle1m";
    // startOKX(symbols.to_string(), success_callback);

    println!("connected... ");
    let _ = ctrl_c().await;
    println!("Received Ctrl+C, shutting down...");
}

fn startBinance(symbols: String, success_callback: impl FnOnce(String) + Send + 'static) {
    let binance_thread = thread::spawn(move || {
        let rt = match Builder::new_multi_thread()
            .worker_threads(1) // Only 1 thread for Binance
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("Error building runtime: {}", e);
                return; // Or handle the error as needed
            }
        };

        rt.block_on(async {
            println!("Started Binance thread: {:?}", std::thread::current().id());
            match Binance::connect(&symbols, success_callback).await {
                Ok(msg) => println!("{:?}", msg),
                Err(err) => eprint!("{:?}", err),
            }
        });
    });
}

fn startOKX(symbols: String, success_callback: impl FnOnce(String) + Send + 'static) {
    let okx_thread = thread::spawn(move || {
        let rt = match Builder::new_multi_thread()
            .worker_threads(1) // Only 1 thread for Binance
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("Error building runtime: {}", e);
                return; // Or handle the error as needed
            }
        };

        rt.block_on(async {
            println!("Started OKX thread: {:?}", std::thread::current().id());
            match OKX::connect(&symbols, success_callback).await {
                Ok(msg) => print!("{:?}", msg),
                Err(err) => eprint!("{:?}", err),
            }
        });
    });
}
