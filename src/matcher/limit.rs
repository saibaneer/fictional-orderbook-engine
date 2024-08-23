use crate::matcher::order::Order;
use crate::matcher::utils::Price;

use super::types::BidOrAsk;
// use crate::matcher::

#[derive(Debug)]
pub struct Limit {
    pub price: Price,
    pub orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: f64) -> Self {
        Limit {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        if self.orders.is_empty() || self.orders[0].order_type == order.order_type {
            self.orders.push(order);
        } else {
            eprintln!("This limit cannot be filled with this type of order");
        }
    }

    fn total_volume(&self) -> f64 {
        return self
            .orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap();
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        if self.orders.is_empty() {
            eprintln!("There are no orders to fill");
            return;
        }

        if self.orders[0].order_type == market_order.order_type {
            eprintln!("This limit cannot be filled with this type of order");
            return;
        }

        for order in self.orders.iter_mut() {
            if market_order.is_filled() {
                break;
            }
            if market_order.size >= order.size {
                // Market order consumes the entire limit order
                market_order.size -= order.size;
                order.size = 0.0;
            } else {
                // Limit order partially fills the market order
                order.size -= market_order.size;
                market_order.size = 0.0;
            }
        }

        // Remove fully filled orders
        self.orders.retain(|order| order.size > 0.0);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::matcher::types::BidOrAsk;

    use super::*;

    #[test]
    fn test_limit_total_volume() {
        let actual_price = 105.5;
        let mut limit = Limit::new(actual_price);

        let buy_limit_order_1 = Order::new(BidOrAsk::Bid, 50.0);
        let buy_limit_order_2 = Order::new(BidOrAsk::Bid, 50.0);
        let buy_limit_order_3 = Order::new(BidOrAsk::Bid, 50.0);

        limit.add_order(buy_limit_order_1);
        limit.add_order(buy_limit_order_2);
        limit.add_order(buy_limit_order_3);

        assert_eq!(limit.total_volume(), 150.0)
    }
    #[test]
    fn test_limit_fill_small_market_order() {
        let actual_price = 105.5;
        let mut limit = Limit::new(actual_price);

        // Create and add a limit order (buy order) to the limit
        let buy_limit_order = Order::new(BidOrAsk::Bid, 450.0);
        limit.add_order(buy_limit_order);

        // Create a small market sell order
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 50.0);

        // Fill the market order
        limit.fill_order(&mut market_sell_order);

        // Assert that the market order size was reduced to 0.0
        assert_eq!(market_sell_order.size, 0.0);

        // Assert that the first order in the limit has been reduced by 50.0
        assert_eq!(limit.orders[0].size, 400.0);

        // Optionally, print the state for debugging
        println!("{:?}", limit);
    }

    #[test]
    fn test_fill_no_orders() {
        //Setup
        let actual_price = 105.5;
        let mut limit = Limit::new(actual_price);

        // Create a market sell order
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 50.0);

        //Action
        limit.fill_order(&mut market_sell_order);

        //Assert
        assert_eq!(market_sell_order.size, 50.0);
        assert_eq!(limit.orders.len(), 0);
    }

    #[test]
    fn test_fill_incompatible_types() {
        //Setup
        let actual_price = 105.5;
        let mut limit = Limit::new(actual_price);

        // Add a buy order to the limit
        let buy_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_order);

        // Create a market buy order (incompatible with buy orders in the limit)
        let mut market_buy_order = Order::new(BidOrAsk::Bid, 50.0);

        // Fill the market order
        limit.fill_order(&mut market_buy_order);

        // The market order size should remain unchanged due to incompatibility
        assert_eq!(market_buy_order.size, 50.0);

        // The limit order size should remain unchanged
        assert_eq!(limit.orders[0].size, 100.0);
    }

    #[test]
    fn test_fill_order_fully_filled_market_order() {
        let actual_price = 105.5;
        let mut limit = Limit::new(actual_price);

        // Add a buy order to the limit
        let buy_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_order);

        // Create a market sell order that fully fills the limit order
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 100.0);

        // Fill the market order
        limit.fill_order(&mut market_sell_order);

        // The market order size should be reduced to 0.0
        assert_eq!(market_sell_order.size, 0.0);

        // The limit order size should be reduced to 0.0
        assert!(limit.orders.len() == 0);
    }

    #[test]
    fn test_fill_order_partially_filled_market_order() {
        let actual_price = 105.5;
        let mut limit = Limit::new(actual_price);

        // Add multiple buy orders to the limit
        limit.add_order(Order::new(BidOrAsk::Bid, 100.0));
        limit.add_order(Order::new(BidOrAsk::Bid, 200.0));

        // Create a market sell order that partially fills the first limit order
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 150.0);

        // Fill the market order
        limit.fill_order(&mut market_sell_order);

        // The market order should be fully filled
        assert_eq!(market_sell_order.size, 0.0);

        // The second limit order should be partially filled
        assert_eq!(limit.orders[0].size, 150.0);
    }
}
