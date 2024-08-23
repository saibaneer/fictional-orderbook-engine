use super::order::Order;
use crate::matcher::types::TradingPair;
use crate::Orderbook;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: f64,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(existing_orderbook) => {
                existing_orderbook.add_order(price, order);
                println!("placed limit order at price level {:?}", price);
                Ok(())
            }
            None => {
                // todo!() //create if none-existing
                // self.add_new_market(pair.clone());
                // format!("Market created for trading pair {:?}", &pair.to_string());
                Err(format!(
                    "the order book for the given trading pair {} does not exist",
                    pair.to_string()
                ))
            }
        }
    }
}
