use core_orderbook::{Order, OrderBook, OrderType};

fn main() {
    let mut order_book = OrderBook {
         buy_orders: Vec::new(),
         sell_orders: Vec::new() };

    println!("--- Test Case: Canceling an Order ---");

    // 1. Add some orders with unique IDs
    let buy_order_1 = Order {
        id: 1,
        order_type: OrderType::Buy,
        price: 10.0,
        quantity: 100,
    };
    let buy_order_2 = Order {
        id: 2,
        order_type: OrderType::Buy,
        price: 10.5,
        quantity: 50,
    };
    let sell_order_1 = Order {
        id: 3,
        order_type: OrderType::Sell,
        price: 11.0,
        quantity: 75,
    };
    let sell_order_2 = Order {
        id: 4,
        order_type: OrderType::Sell,
        price: 11.5,
        quantity: 25,
    };
    
    order_book.add_order(&buy_order_1);
    order_book.add_order(&buy_order_2);
    order_book.add_order(&sell_order_1);
    order_book.add_order(&sell_order_2);

    println!("\nInitial State:");
    println!("Buy Orders: {:?}", order_book.buy_orders);
    println!("Sell Orders: {:?}", order_book.sell_orders);

    // 2. Cancel a buy order
    let order_id_to_cancel = 2;
    println!("\nCanceling order with ID: {}", order_id_to_cancel);
    order_book.cancel_order(order_id_to_cancel, OrderType::Buy);

    // 3. Display the final state of the order book
    println!("\nFinal state after canceling the buy order:");
    println!("Buy Orders: {:?}", order_book.buy_orders);
    println!("Sell Orders: {:?}", order_book.sell_orders);
}
