use super::domain_entity::Order;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::collections::VecDeque;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

pub struct OrderDAG {
    pub graph: DiGraph<Order, ()>, // Using unit type for edge weights as they're not needed here.
    pub indices: HashMap<u64, NodeIndex>, // Mapping from event ID to NodeIndex for quick access.
}

pub struct OrderExecutor {
    pub dag: OrderDAG,
    pub status_tx: Sender<String>, // Channel to send status updates to the main thread
}

pub enum Notification {
    Slack { message: String, channel: String },
}

pub struct MarketDataConnector {
    pub api_key: String,
    pub ws_url: String,
    pub status_tx: Sender<String>,
}

pub struct NotificationService {
    pub webhook_url: String,
    pub rx: Receiver<Notification>,
}
