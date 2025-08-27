use core_orderbook::{Order,OrderBook};

fn main() {
 let mut order_book = OrderBook {
        buy_orders: Vec::new(),
        sell_orders: Vec::new(),
    };

    let order1 = Order {
        id: 1,
        order_type: core_orderbook::OrderType::Buy,
        price: 100.5,
        quantity: 10,
    };

    let order2 = Order {
        id: 2,
        order_type: core_orderbook::OrderType::Sell,
        price: 101.0,
        quantity: 5,
    };

    order_book.add_order(&order1);
    order_book.add_order(&order2);

    order_book.display_order();
}