#[derive(Clone, Debug, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    pub id: u32,
    pub order_type: OrderType,
    pub price: f64,
    pub quantity: u32,
}
pub struct OrderBook {
    pub buy_orders: Vec<Order>,
    pub sell_orders: Vec<Order>,
}

impl OrderBook {
    pub fn add_order(&mut self, order: &Order) {
        match order.order_type {
            OrderType::Buy => self.buy_orders.push(order.clone()),
            OrderType::Sell => self.sell_orders.push(order.clone()),
        }
        self.sorting_orders();
        if self.buy_orders.is_empty() || self.sell_orders.is_empty() {
            return;
        } else {
            let highest_buy = self.buy_orders.first();
            let lowest_sell = self.sell_orders.first();
            if let (Some(high), Some(low)) = (highest_buy, lowest_sell) {
                if high.price >= low.price {
                    println!("Matching orders found:");
                    println!("Buy Order: {:?}", high);
                    println!("Sell Order: {:?}", low);
                }
            }
        }

    }

    pub fn display_order(&self) {
        println!("--- Buy Orders ---");
        for order in &self.buy_orders {
            println!("{:?}", order);
        }
        println!("\n--- Sell Orders ---");
        for order in &self.sell_orders {
            println!("{:?}", order);
        }
    }
    pub fn sorting_orders(&mut self) {
        self.buy_orders
            .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        self.sell_orders
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    }
}
