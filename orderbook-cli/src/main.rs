use core_orderbook::{Order, OrderBook, OrderType};

fn main() {
    let mut order_book = OrderBook {
        buy_orders: Vec::new(),
        sell_orders: Vec::new(),
    };

    println!("Adding 20 orders to the order book...");

    // Create 20 orders in a loop
    for i in 1..=20 {
        let order_id = i;
        let price = if i % 2 == 0 { 9.5 + i as f64 * 0.1 } else { 10.0 - i as f64 * 0.1 };
        let order_type = if i % 2 == 0 { OrderType::Buy } else { OrderType::Sell };
        
        let new_order = Order {
            id: order_id,
            order_type: order_type,
            price: price,
            quantity: 5,
        };
        
        println!("\nAdding new order: {:?}", new_order);
        order_book.add_order(&new_order);
    }
    
    // Display the final state of the order book
    println!("\nFinal state of the order book:");
    order_book.display_order();
}