use core_orderbook::{Order, OrderBook, OrderType};

fn main() {
    let mut order_book = OrderBook {
        buy_orders: Vec::new(),
        sell_orders: Vec::new(),
    };

    println!("Initial state of the order book:");
    order_book.display_order();

    println!("\n--- Test Case: Partial Fill and Complete Fill ---");

    // 1. Add a large buy order
    let buy_order_large = Order {
        id: 1,
        order_type: OrderType::Buy,
        price: 10.0,
        quantity: 100,
    };
    println!("\nAdding new order: {:?}", buy_order_large);
    order_book.add_order(&buy_order_large);

    // 2. Add a small sell order that partially fills the buy order
    let sell_order_small = Order {
        id: 2,
        order_type: OrderType::Sell,
        price: 10.0,
        quantity: 50,
    };
    println!("\nAdding new order: {:?}", sell_order_small);
    order_book.add_order(&sell_order_small);

    // 3. Add another small sell order to fully fill the remaining quantity
    let sell_order_fill = Order {
        id: 3,
        order_type: OrderType::Sell,
        price: 10.0,
        quantity: 50,
    };
    println!("\nAdding new order: {:?}", sell_order_fill);
    order_book.add_order(&sell_order_fill);
    
    // 4. Add a non-matching order to show the loop breaks
    let sell_order_break = Order {
        id: 4,
        order_type: OrderType::Sell,
        price: 11.0,
        quantity: 20,
    };
    println!("\nAdding new order: {:?}", sell_order_break);
    order_book.add_order(&sell_order_break);

    // 5. Display the final state of the order book
    println!("\nFinal state of the order book:");
    order_book.display_order();
}
