use std::thread;
use tokio::time::{sleep, Duration};
use tokio::runtime::Builder;

use MarketDataFeed::binance::{Binance};
use MarketDataFeed::common::ExchangeFeed;
use MarketDataFeed::okx::OKX;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    let success_callback = |message: String| {
        println!("Success callback received message: {}", message);
    };
    
    let configPath = "configs/config.toml".to_string();
    let binanceConfig = &configPath;
    let okxConfig = &configPath;

    // let symbols = "btcusdt@kline_1m";
    startBinance( success_callback, binanceConfig.to_string());

    // let symbols = "BTC-USDT@candle1m";
    startOKX(success_callback, okxConfig.to_string());

    let _ = ctrl_c().await;
    println!("Received Ctrl+C, shutting down...");
}

fn startBinance(success_callback: impl FnOnce(String) + Send + Clone + Copy + 'static, config_path : String) {
    let binance = Binance::new(&config_path).unwrap();
    if !binance.enable {
        println!("Binance disabled.");
        return;
    }

    let binance_thread = thread::spawn(move || {
        let rt: tokio::runtime::Runtime = match Builder::new_multi_thread()
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
            let max_retries = 5; // Maximum retry attempts
            let mut retries = 0;

            while retries < max_retries {
                let callback = success_callback.clone();
                match Binance::connect(&binance, callback).await {
                    Ok(msg) => {
                        println!("{:?}", msg);
                        return; // If connection is successful, exit the loop
                    }
                    Err(err) => {
                        eprint!("Error: {:?}. Retrying...\n", err);
                        retries += 1;
                        if retries < max_retries {
                            // Introduce a delay before retrying
                            let delay = Duration::from_secs(30);
                            sleep(delay).await;
                        } else {
                            eprintln!("Max retries reached. Exiting...");
                            return; // Exit after max retries
                        }
                    }
                }
            }
        });
    });
}

fn startOKX( success_callback: impl FnOnce(String) + Send + 'static, config_path : String) {
    let okx = OKX::new(&config_path).unwrap();
    if !okx.enable {
        println!("OKX disabled.");
        return;
    }

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
            match OKX::connect(&okx, success_callback).await {
                Ok(msg) => print!("{:?}", msg),
                Err(err) => eprint!("{:?}", err),
            }
        });
    });
}
