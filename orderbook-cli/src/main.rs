use core_orderbook::{Order, OrderBook, OrderType};

fn main() {
    let mut order_book = OrderBook {
        buy_orders: Vec::new(),
        sell_orders: Vec::new(),
    };

    println!("--- Test Case: Canceling an Order ---");

    // 1. Add a buy order and a sell order that do not match
    let buy_order = Order {
        id: 1,
        order_type: OrderType::Buy,
        price: 10.0,
        quantity: 100,
    };
    order_book.add_order(&buy_order);
    let buy_order = Order {
        id: 2,
        order_type: OrderType::Buy,
        price: 10.0,
        quantity: 100,
    };
    let sell_order = Order {
        id: 3,
        order_type: OrderType::Sell,
        price: 11.0,
        quantity: 50,
    };
     order_book.add_order(&sell_order);
    let sell_order = Order {
        id: 3,
        order_type: OrderType::Sell,
        price: 11.0,
        quantity: 50,
    };

    println!("\nInitial State: Adding buy and sell orders that do not match.");
    order_book.add_order(&buy_order);
    order_book.add_order(&sell_order);
    
   
    order_book.display_order();

    // 2. Cancel the buy order
    let order_id_to_cancel = 2;
    println!("\nCanceling order with ID: {}", order_id_to_cancel);
    order_book.cancel_order(order_id_to_cancel, OrderType::Buy);

    // 3. Display the final state of the order book
    println!("\nFinal state after canceling the buy order:");
    order_book.display_order();
}
