use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::VecDeque;

pub enum EventType {
    OrderPlaced(Order<Box<dyn Tradable>>),
    OrderCancelled(u64),
    TradeExecuted(Trade),
    MarketDataUpdated(MarketData),
}

pub struct Event {
    event_type: EventType,
    timestamp: u64,
    dependencies: Vec<u64>,
}

pub struct EventDAG {
    graph: DiGraph<Event, ()>, // Using unit type for edge weights as they're not needed here.
    indices: HashMap<u64, NodeIndex>, // Mapping from event ID to NodeIndex for quick access.
}

pub struct EventProcessor {
    dag: EventDAG,
    ready_events: VecDeque<u64>, // Queue of IDs of events ready to be processed
}
