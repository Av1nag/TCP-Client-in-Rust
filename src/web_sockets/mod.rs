use binance_spot_connector_rust::{
    market_stream::trade::TradeStream, tokio_tungstenite::BinanceWebSocketClient,
};
use chrono::prelude::*;
use futures_util::StreamExt;
use std::{io::Write, time::Duration};

use crate::data_to_file_append;
#[allow(unused_mut)]
#[allow(unused_variables)]
#[tokio::main]
pub async fn btc_data(no_of_times: u32) -> (f64, u32, f64) {
    let start_time = tokio::time::Instant::now();

    // Builder::from_default_env()
    //     .filter(None, log::LevelFilter::Info)
    //     .init();
    // Establish connection
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![&TradeStream::new("BTCUSDT").into()])
        .await;
    // Read messages
    let mut message_counter = 0;
    let mut sum_of_the_prices: f64 = 0.0;
    let mut average_price: f64 = 0.00;
    // Set a timer for '_' seconds
    let timeout_duration = Duration::from_secs(no_of_times.into());

    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                let binary_data = message.into_data();
                let data = std::str::from_utf8(&binary_data).expect("Failed to parse message");
                let parsed_data: serde_json::Value =
                    serde_json::from_str(&data).expect("Unable to parse message");

                if let Some(price) = parsed_data["data"]["p"].as_str() {
                    let timestamp = Utc::now().format("Date: %Y-%m-%d, Time:%H:%M:%S");
                    let log_message = format!("[{}] BTC Price: {}", timestamp, price);
                    let data_parser = data_to_file_append::main();
                    // Write to a file
                    writeln!(&data_parser.1, "{}", log_message).expect("write failed");
                    match price.parse::<f64>() {
                        Ok(parsed_price) => sum_of_the_prices += parsed_price,
                        Err(e) => log::error!("error, failed in parising {}", e),
                    }

                    message_counter += 1;
                }
                average_price = sum_of_the_prices / message_counter as f64;

                if tokio::time::Instant::now() - start_time >= timeout_duration {
                    println!(
                        "Cache complete. The average USD price of BTC is: {}",
                        &average_price
                    );
                    break;
                }
            }
            Err(_) => break,
        }
    }
    // Disconnect
    conn.close().await.expect("Failed to disconnect");
    (sum_of_the_prices, message_counter, average_price)
}
