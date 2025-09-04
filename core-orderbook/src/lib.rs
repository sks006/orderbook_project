/// TODO:STEP:1 Represents the type of an order.
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
    /// TODO: STEP:2   Adds a new order to the order book.
    pub fn add_order(&mut self, order: &Order) {
        match order.order_type {
            OrderType::Buy => self.buy_orders.push(order.clone()),
            OrderType::Sell => self.sell_orders.push(order.clone()),
        }
        self.sorting_orders();

        // TODO: loop through the orders and match them
        while let (Some(high), Some(low)) =
            (self.buy_orders.get_mut(0), self.sell_orders.get_mut(0))
        {
            if high.price >= low.price {
                let filled_quantity = std::cmp::min(high.quantity, low.quantity);

                high.quantity -= filled_quantity;
                low.quantity -= filled_quantity;

                println!("Matching orders found!");
                println!("Filled quantity: {}", filled_quantity);
                println!("Remaining Buy Order: {:?}", high);
                println!("Remaining Sell Order: {:?}", low);

                if high.quantity == 0 {
                    self.buy_orders.remove(0);
                }

                if low.quantity == 0 {
                    self.sell_orders.remove(0);
                }
            } else {
                break;
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

    /// TODO: STEP:3 Sorts the orders in the order book.
    pub fn sorting_orders(&mut self) {
        self.buy_orders
            .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        self.sell_orders
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    }

    // TODO: STEP:4 cancel order
    pub fn cancel_order(&mut self, order_id: u32, order_type: OrderType) {
        match order_type {
            OrderType::Buy => self.buy_orders.retain(|order| order.id != order_id),
            OrderType::Sell => self.sell_orders.retain(|order| order.id != order_id),
        }
    }

    pub fn get_best_bid_and_ask(&self) -> (Option<f64>, Option<f64>) {
        // Find the best bid (highest buy price)
        let best_bid = self
            .buy_orders
            .iter()
            .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
            .map(|order| order.price);

        // Find the best ask (lowest sell price)
        let best_ask = self
            .sell_orders
            .iter()
            .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
            .map(|order| order.price);

        (best_bid, best_ask)
    }
}
