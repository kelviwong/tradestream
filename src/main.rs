use std::thread;
use std::{thread::sleep, time::Duration};
use tokio::runtime::Builder;

use MarketDataFeed::binance::{Binance, ExchangeFeed};
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    let symbols = "btcusdt@kline_1m";
    // match Binance::connect(symbols).await {
    //     Ok(handle) => {
    //         println!("Binance connection task spawned!");
    //         handle.await.unwrap(); // Handle task completion
    //         println!("Binance connection task completed.");
    //     }
    //     Err(err) => {
    //         println!("Failed to connect: {}", err);
    //     }
    // }

    let binance_thread = thread::spawn(|| {
        let rt = Builder::new_multi_thread()
            .worker_threads(1) // Only 1 thread for Binance
            .enable_all()
            .build()
            .unwrap();
        

        rt.block_on(async {
            Binance::connect(symbols).await;
            // match Binance::connect(symbols).await {
            //     Ok(handle) => {
            //         println!("Binance connection task spawned!");
            //         handle.await.unwrap(); // Handle task completion
            //         println!("Binance connection task completed.");
            //     }
            //     Err(err) => {
            //         println!("Failed to connect: {}", err);
            //     }
            // }
        });
    });

    // Binance::start_connection_spin(symbols).await;
    println!("connected... ");
    let _ = ctrl_c().await;
    println!("Received Ctrl+C, shutting down...");
    // binance_task.await.unwrap()
    // let _ = tokio::try_join!(binance_task);
    // match Binance::connect(symbols).await {
    //     Ok(connection_str) => println!("Connected: {}", connection_str),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
}
