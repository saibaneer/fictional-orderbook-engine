use crate::matcher::limit::Limit;
use crate::matcher::order::Order;
use crate::matcher::types::BidOrAsk;
use crate::matcher::utils::Price;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    pub fn new() -> Self {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let order_price = Price::new(price);
        match order.order_type {
            BidOrAsk::Ask => match self.asks.get_mut(&order_price) {
                Some(limit) => limit.orders.push(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(order_price, limit);
                }
            },
            BidOrAsk::Bid => {
                // let limit = self.bids.get_mut(&order_price);
                match self.bids.get_mut(&order_price) {
                    Some(limit) => limit.orders.push(order),
                    None => {
                        // todo!()
                        // let dereferenced_limit = limit.unwrap();
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(order_price, limit);
                    }
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add_order() {
//         let mut orderbook = Orderbook::new();
//         let order = Order::new(BidOrAsk::Bid, 50.0);
//         orderbook.add_order(5.0, order);
//         assert_eq!(orderbook.bids.len(), 1);
//     }
// }
