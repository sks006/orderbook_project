# Rust Order Book 
# High-Performance Order Book

This project is a robust, high-performance order book implementation written in Rust. It is designed to handle a large volume of concurrent buy and sell orders with minimal latency. The architecture prioritizes thread-safe operations, memory efficiency, and a clear separation of concerns to provide a reliable and scalable trading system.

### Features

* **Concurrent & Lock-Free Design:** Utilizes `DashMap` for efficient, concurrent access to price levels without needing fine-grained locks.
* **High-Performance Matching Engine:** A sophisticated matching engine that handles limit and market orders, with optimizations like memory pooling to reduce allocation overhead.
* **Flexible Order Types:** Supports various order types, including standard limit orders, iceberg orders, and post-only orders.
* **Efficient Lookups:** The use of `BTreeMap`-like behavior via `DashMap` ensures fast retrieval of best bid and ask prices.
* **Caching & Optimization:** Includes a caching layer for the best bid and ask prices using atomic types, providing instant access to critical market data.
* **Robust Error Handling:** Custom error types are defined to clearly communicate specific issues, such as insufficient liquidity or invalid operations.
* **Order Book Snapshots:** A mechanism to create a read-only snapshot of the order book's state, useful for market data feeds and analysis.

### Core Components

* `book.rs`:
    * **Purpose:** The central hub of the project. It defines the `OrderBook` struct and its main methods for managing bid and ask sides. It uses `DashMap` to store orders, keyed by price.

* `matching.rs`:
    * **Purpose:** Contains the core logic for the matching engine. It implements highly optimized functions to match incoming orders with existing ones on the opposite side of the book. It leverages a memory pool to minimize allocations.

* `modifications.rs`:
    * **Purpose:** Handles all state-changing operations on the order book, such as adding, canceling, and updating orders. It uses traits to abstract the logic for different order types.

* `operations.rs`:
    * **Purpose:** Provides the public-facing API for users. This is where high-level functions like `add_limit_order`, `add_iceberg_order`, and `submit_market_order` are defined for easy interaction with the order book.

* `cache.rs`:
    * **Purpose:** Manages a cache for the best bid and ask prices. It uses `AtomicU64` to store these prices, ensuring that they can be read and updated concurrently and without locks, making them instantly available.

* `error.rs`:
    * **Purpose:** Defines all the custom error types for the project. This is crucial for providing clear and descriptive error messages to the user.

* `pool.rs`:
    * **Purpose:** Implements a memory pooling strategy. It pre-allocates and reuses vectors for common operations, significantly reducing the overhead of memory management in a high-frequency context.

* `snapshot.rs`:
    * **Purpose:** Contains the logic for creating a complete, read-only snapshot of the order book at any given moment. This is essential for providing market data to external systems.

* `private.rs`:
    * **Purpose:** A module for internal, private helper functions and unit tests.

* `mod.rs`:
    * **Purpose:** The root module of the library. It organizes all the sub-modules and defines their public/private access, acting as the main entry point for the library.

### Getting Started

To get started, you can add the library to your `Cargo.toml` and interact with the `OrderBook` struct.

```rust
use orderbook::{OrderBook, OrderType, Side, TimeInForce};
use uuid::Uuid;

fn main() {
    // Create a new order book
    let order_book = OrderBook::new("BTC-USD".to_string());

    // Add a limit buy order
    let buy_id = Uuid::new_v4().to_string();
    let buy_order = order_book.add_limit_order(&buy_id, 10000, 100, Side::Buy, TimeInForce::Gtc).unwrap();
    println!("Added buy order: {:?}", buy_order);

    // Add a limit sell order that matches the buy order
    let sell_id = Uuid::new_v4().to_string();
    let match_result = order_book.submit_limit_order(sell_id, 10000, 50, Side::Sell, TimeInForce::Gtc).unwrap();
    println!("Match result: {:?}", match_result);

    // Get the current best bid and ask
    let (best_bid, best_ask) = order_book.get_best_bid_and_ask();
    println!("Best Bid: {:?}, Best Ask: {:?}", best_bid, best_ask);
}for the application. It contains a sample test case that adds several orders to the order book to demonstrate how the matching and partial fill logic works.
