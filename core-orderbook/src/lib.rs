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
}
