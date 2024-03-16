mod system_entity;
use petgraph::algo::toposort;
use std::collections::VecDeque;
use tokio::task;

impl system_entity::Event {
    pub fn new(
        event_type: system_entity::EventType,
        timestamp: u64,
        dependencies: Vec<u64>,
    ) -> Self {
        Self {
            event_type,
            timestamp,
            dependencies,
        }
    }
}

impl EventDAG {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            indices: HashMap::new(),
        }
    }

    // Adds an event to the graph and sets up dependencies.
    pub fn add_event(&mut self, event: Event, dependencies: Vec<u64>) {
        let node_idx = self.graph.add_node(event);
        self.indices.insert(event.id, node_idx);

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
                    .push(self.graph[node].id);
            }
        }

        // Convert HashMap to Vec<Vec<u64>> while preserving order based on levels.
        let mut grouped_events: Vec<Vec<u64>> = levels.into_iter().map(|(_, v)| v).collect();
        grouped_events.sort_by_key(|v| v[0]);
        grouped_events
    }
}

impl EventProcessor {
    pub async fn process_events(dag: &EventDAG) {
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
            }
        }
    }
}
