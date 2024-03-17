#![warn(rust_2018_idioms)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

mod descriptor;
mod market_listener;

use descriptor::system_logic::MarketDataConnector;
use descriptor::system_logic::NotificationService;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Notification>(32);
    let ploygon_api_key = env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY is not set");
    let polygon_ws_url = env::var("POLYGON_WS_URL").expect("POLYGON_WS_URL is not set");
    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    let notification_service = NotificationService::new(slack_webhook_url, rx);

    tokio::spawn(async move {
        notification_service.listen_and_send().await;
    });

    let market_data_connector = MarketDataConnector::new(ploygon_api_key, polygon_ws_url, tx);
    tokio::spawn(async move {
        market_data_connector.connect().await;
    });

    let order_dag = OrderDAG::new();
    let order_executor = OrderExecutor::new(order_dag, tx.clone());
    order_executor.process_events().await;
}
