use crate::matcher::order::Order;
use crate::matcher::utils::Price;
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
        // let new_order = Order::new(order_type, size);
        self.orders.push(order);
    }

    // pub fn total_volume(&self, order_type: BidOrAsk) -> f64 {
    //     for order in self.orders.iter() {

    //     }
    // }
 
    pub fn fill_order(&mut self, market_order: &mut Order) {
        for order in self.orders.iter_mut() {
            match market_order.size >= order.size {
                true => {
                    market_order.size -= order.size;
                    order.size = 0.0;
                    println!("Market order size: {}", &market_order.size);
                }
                false => {
                    order.size -= market_order.size;
                    market_order.size = 0.0;
                    break
                }  
            }
        }
        // if market_order.size
    }
}


#[cfg(test)]
pub mod tests {
    use crate::matcher::types::BidOrAsk;

    use super::*;
    #[test]
    fn test_limit_fill_small_market_order(){
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
    fn test_limit_fill_large_market_order(){
        let actual_price = 105.5;
        let mut limit = Limit::new(actual_price);

        // Create and add a limit order (buy order) to the limit
        let buy_limit_order = Order::new(BidOrAsk::Bid, 50.0);
        limit.add_order(buy_limit_order);

        // Create a small market sell order
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 150.0);

         // Fill the market order
         limit.fill_order(&mut market_sell_order);

         // Optionally, print the state for debugging
        println!("{:?}", limit);

    }
}
