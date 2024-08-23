

#[cfg(test)]
mod tests {
    use matching_engine::matcher::orderbook::Orderbook;
    use matching_engine::matcher::order::Order;
    use matching_engine::matcher::order_type::BidOrAsk;
    use matching_engine::matcher::price::Price;

    #[test]
    fn test_add_bid_order() {
        let mut orderbook = Orderbook::new();
        let bid_order = Order::new(BidOrAsk::Bid, 50.0);

        orderbook.add_order(5.0, bid_order);

        assert_eq!(orderbook.bids.len(), 1);
        assert!(orderbook.bids.contains_key(&Price::new(5.0)));
    }

    #[test]
    fn test_add_ask_order() {
        let mut orderbook = Orderbook::new();
        let ask_order = Order::new(BidOrAsk::Ask, 30.0);

        orderbook.add_order(10.0, ask_order);

        assert_eq!(orderbook.asks.len(), 1);
        assert!(orderbook.asks.contains_key(&Price::new(10.0)));
    }

    #[test]
    fn test_add_multiple_orders() {
        let mut orderbook = Orderbook::new();
        let bid_order1 = Order::new(BidOrAsk::Bid, 20.0);
        let bid_order2 = Order::new(BidOrAsk::Bid, 30.0);
        let ask_order = Order::new(BidOrAsk::Ask, 15.0);

        orderbook.add_order(5.0, bid_order1);
        orderbook.add_order(5.0, bid_order2);
        orderbook.add_order(10.0, ask_order);

        assert_eq!(orderbook.bids.len(), 1);
        assert_eq!(orderbook.bids.get(&Price::new(5.0)).unwrap().orders.len(), 2);

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.asks.get(&Price::new(10.0)).unwrap().orders.len(), 1);
    }
}