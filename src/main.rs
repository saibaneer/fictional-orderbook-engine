use std::collections::HashMap;

#[derive(Debug)]
struct Order {
    size: f64,
    order_type: BidOrAsk,
}

impl Order {
    fn new(order_type: BidOrAsk, order_size: f64) -> Self {
        Order {
            size: order_size,
            order_type,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Price {
    integer: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Self {
        let scalar = 100000;
        let integer = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integer,
            fractional,
            scalar,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: f64) -> Self {
        Limit {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        // let new_order = Order::new(order_type, size);
        self.orders.push(order);
    }
}

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    fn new() -> Self {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self,price: f64,  order: Order) {
        let order_price = Price::new(price);
          match order.order_type {
            BidOrAsk::Ask => {
                todo!()
            }
            BidOrAsk::Bid => {
                // let limit = self.bids.get_mut(&order_price);
                match self.bids.get_mut(&order_price) {
                    Some(limit) => {
                        limit.orders.push(order)
                    }
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

fn main() {
    let price = Price::new(23.4);
    println!("{:?}", price);

    let mut limit = Limit::new(23.4);
    println!("{:?}", &limit);

    // limit.add_order(500.5, BidOrAsk::Ask);
    // limit.add_order(200.0, BidOrAsk::Bid);
    println!("{:?}", &limit);
}
