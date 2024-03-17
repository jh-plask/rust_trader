use crate::descriptor::domain_entity::Order;
use crate::descriptor::system_entity::{
    MarketDataConnector, Notification, NotificationService, OrderDAG, OrderExecutor,
};
use futures::stream::Stream;
use futures::stream::StreamExt;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task;

impl OrderDAG {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            indices: HashMap::new(),
        }
    }

    // Adds an event to the graph and sets up dependencies.
    pub fn add_order(&mut self, order: Order, dependencies: Vec<u64>) {
        let node_idx = self.graph.add_node(order);
        self.indices.insert(order.id.as_u128() as u64, node_idx);

        for dep_id in dependencies {
            if let Some(&dep_idx) = self.indices.get(&dep_id) {
                self.graph.add_edge(dep_idx, node_idx, ());
            }
        }
    }

    // Groups events for parallel execution.
    pub fn group_events_for_parallel_execution(&self) -> Vec<Vec<u64>> {
        let mut levels: HashMap<usize, Vec<u64>> = HashMap::new();
        if let Ok(sorted_nodes) = toposort(&self.graph, None) {
            for node in sorted_nodes {
                let level = self
                    .graph
                    .neighbors_directed(node, petgraph::Direction::Incoming)
                    .map(|n| self.graph[n].level)
                    .max()
                    .unwrap_or(0)
                    + 1;

                self.graph[node].level = level;
                levels
                    .entry(level)
                    .or_insert_with(Vec::new)
                    .push(self.graph[node].id.as_u128() as u64); // Convert Uuid to u64
            }
        }

        // Convert HashMap to Vec<Vec<u64>> while preserving order based on levels.
        let mut grouped_events: Vec<Vec<u64>> = levels.into_iter().map(|(_, v)| v).collect();
        grouped_events.sort_by_key(|v| v[0]);
        grouped_events
    }
}

impl OrderExecutor {
    pub fn new(dag: OrderDAG, status_tx: mpsc::Sender<String>) -> Self {
        Self { dag, status_tx }
    }
    pub async fn process_events(dag: &OrderDAG) {
        let groups = dag.group_events_for_parallel_execution();

        for group in groups.into_iter() {
            // Collect all async tasks for the current group.
            let tasks: Vec<_> = group
                .into_iter()
                .map(|event_id| {
                    task::spawn(async move {
                        // Simulate processing an event. Replace with actual logic.
                        println!("Processing event ID: {}", event_id);
                        // Example of a possible async operation
                        // e.g., tokio::time::sleep(Duration::from_millis(100)).await;
                    })
                })
                .collect();

            // Await all tasks in the group to ensure they complete before processing the next group.
            for current_task in tasks {
                // Await each task to complete. Handle or log errors as needed.
                let _ = current_task
                    .await
                    .expect("Task panicked or encountered an error.");

                // Send status updates to the main thread.
                let _ = self
                    .status_tx
                    .send(Notification::Slack {
                        message: format!("Processed event ID: {}", event_id),
                        channel: "operations".to_string(),
                    })
                    .await
                    .expect("Failed to send status update");
            }
        }
    }
}

impl MarketDataConnector {
    pub fn new(api_key: String, ws_url: String, status_tx: mpsc::Sender<String>) -> Self {
        Self {
            api_key,
            ws_url,
            status_tx,
        }
    }

    pub async fn connect(&self) {
        let url = format!("{}?apiKey={}", self.ws_url, self.api_key);

        loop {
            match tokio_tungstenite::connect_async(url.clone()).await {
                Ok((_ws_stream, _)) => {
                    // Send a successful connection notification
                    let _ = self
                        .status_tx
                        .send(Notification::Slack {
                            message: "Connected to WebSocket".to_string(),
                            channel: "operations".to_string(),
                        })
                        .await
                        .expect("Failed to send notification");
                }
                Err(e) => {
                    // Send a failed connection notification
                    let _ = self
                        .status_tx
                        .send(Notification::Slack {
                            message: format!("Failed to connect: {}", e),
                            channel: "operations".to_string(),
                        })
                        .await
                        .expect("Failed to send notification");
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                }
            }
        }
    }
}

impl Notification {
    pub async fn send(&self, webhook_url: &str) -> Result<(), reqwest::Error> {
        match self {
            Notification::Slack { message, channel } => {
                let client = Client::new();
                // Constructing the message to include channel information.
                // Note: This is for demonstration. Slack's webhook does not route messages based on this field.
                let formatted_message = format!("Channel: {}\n{}", channel, message);

                client
                    .post(webhook_url)
                    .header("Content-Type", "application/json")
                    .body(json!({"text": formatted_message}).to_string())
                    .send()
                    .await?;
            }
        }
        Ok(())
    }
}

impl NotificationService {
    pub fn new(webhook_url: String, rx: mpsc::Receiver<Notification>) -> Self {
        Self { webhook_url, rx }
    }

    // Changed to directly accept the stream for more flexibility and efficiency
    pub async fn listen_and_send(mut self) {
        while let Some(notification) = self.rx.recv().await {
            match notification.send(&self.webhook_url).await {
                Ok(_) => println!("Notification sent successfully"),
                Err(e) => eprintln!("Failed to send notification: {}", e),
            }
        }
    }
}
