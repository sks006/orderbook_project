Rust Order Book Matching Engine
This project is a simple, yet robust, order book matching engine built in Rust. It serves as a foundational example of how a trading system handles and matches buy and sell orders.

The project is structured into two main parts: the core logic library (lib.rs) and a test runner (main.rs).

📦 Key Features
Order Types: Supports two types of orders: Buy and Sell.

Efficient Matching: Uses a while let loop to continuously check for and execute trades as long as there are matches.

Sorting: Orders are automatically sorted to ensure the best bids (highest price) and asks (lowest price) are at the top of the book.

Partial Fills: Intelligently handles trades where the buy and sell quantities are different, updating the remaining quantities and removing orders only when they are fully filled.

Clean Architecture: Separates the core order book logic from the application's main test case.

🚀 How to Run
To get started, make sure you have Rust and Cargo installed. If not, you can install them using rustup.

Navigate to the project directory in your terminal.

Run the application using Cargo:

cargo run

This command will automatically compile the project and execute the test case found in src/main.rs. The output will demonstrate the order book's ability to handle partial fills and remove completed orders.

📄 File Structure
src/lib.rs: Contains the core OrderBook struct and its methods, including add_order, sorting_orders, and display_order. This file holds the entire business logic of the matching engine.

src/main.rs: The entry point for the application. It contains a sample test case that adds several orders to the order book to demonstrate how the matching and partial fill logic works.
