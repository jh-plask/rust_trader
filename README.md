# Rust Trader

Rust Trader is a high-performance financial trading system implemented in Rust. It leverages Rust's memory safety guarantees and concurrency features to provide a reliable and efficient trading system.

## Project Structure

The project is organized into several modules:

- `src/descriptor/domain_entity.rs`: This file contains the definitions for the domain entities used in the trading system. These entities represent the core business objects in the system, such as orders and trades.

- `src/descriptor/domain_logic.rs`: This file contains the business logic related to the domain entities. This includes the rules for order execution, trade matching, and other core trading operations.

- `src/descriptor/system_entity.rs`: This file contains the definitions for the system entities used in the trading system. These entities represent the infrastructure components of the system, such as the order book and the market data connector.

- `src/descriptor/system_logic.rs`: This file contains the logic related to the system entities. This includes the algorithms for order routing, market data processing, and other system-level operations.

- `src/utils/time.rs`: This file contains utility functions for handling time-related operations. This includes functions for converting between different time formats and calculating time intervals.

- `src/main.rs`: This is the entry point of the application. It sets up the trading system and starts the main event loop.

## Key Features

- **Order Execution**: The `OrderExecutor` struct in `system_entity.rs` is responsible for executing orders. It uses a directed acyclic graph (DAG) to represent the dependencies between orders, allowing for efficient execution of complex order strategies.

- **Market Data Connection**: The `MarketDataConnector` struct in `system_entity.rs` is responsible for connecting to the market data feed. It uses WebSockets to receive real-time market data updates.

- **Notification Service**: The `NotificationService` struct in `system_entity.rs` is responsible for sending notifications. It uses a Slack webhook to send messages to a specified Slack channel.

## How to Run

To run the project, navigate to the [`rust_trader`](command:_github.copilot.openRelativePath?%5B%22rust_trader%22%5D "rust_trader") directory and use the `cargo run` command:

```sh
cd rust_trader
cargo run
```

## How to Test

To run the tests for the project, use the `cargo test` command:

```sh
cargo test
```

## Contributions

Contributions to Rust Trader are welcome! Please submit a pull request or create an issue to discuss any changes you'd like to make.

## License

Rust Trader is licensed under the [MIT License](LICENSE).
