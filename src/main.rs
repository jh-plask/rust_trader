#![warn(rust_2018_idioms)]
use std::env;

use tokio::sync::mpsc;

mod descriptor;

use descriptor::system_entity::{
    MarketDataConnector, Notification, NotificationService, OrderDAG, OrderExecutor,
};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Notification>(32);

    let polygon_api_key = env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY is not set");
    let polygon_ws_url = env::var("POLYGON_WS_URL").expect("POLYGON_WS_URL is not set");
    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    let notification_service = NotificationService::new(slack_webhook_url, rx);
    tokio::spawn(async move {
        notification_service.listen_and_send().await;
    });

    let market_data_connector =
        MarketDataConnector::new(polygon_api_key, polygon_ws_url, tx.clone());
    tokio::spawn(async move {
        market_data_connector.connect().await;
    });

    let order_dag = OrderDAG::new();
    let order_executor = OrderExecutor::new(order_dag, tx);
    order_executor.process_events().await;
}
