mod matcher;
use crate::matcher::order::Order;
use crate::matcher::types::BidOrAsk;
use matcher::engine::MatchingEngine;
use matcher::orderbook::Orderbook;
use matcher::types::TradingPair;

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 50.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 25.5);
    // TO DO: create order endpoint so you can post order with the right auth

    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_from_alice); // in a real world, price would be a GET call.
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order_from_alice = Order::new(BidOrAsk::Ask, 20.5);
    orderbook.add_order(5.0, sell_order_from_alice);

    // println!("{:?}", &orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let new_buy_order = Order::new(BidOrAsk::Bid, 200.0);
    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    let result = engine
        .place_limit_order(pair, 2500.0, new_buy_order)
        .unwrap();
    println!("The result is: {:?}", result);

    println!("Machine engine: {:?}", engine);
}
