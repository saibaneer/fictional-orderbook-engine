use crate::matcher::types::BidOrAsk;

#[derive(Debug, Clone)]
pub struct Order {
    pub size: f64,
    pub order_type: BidOrAsk,
}

impl Order {
    pub fn new(order_type: BidOrAsk, order_size: f64) -> Self {
        Order {
            size: order_size,
            order_type,
        }
    }

    pub fn is_filled(&self) -> bool {
        match self.size == 0.0 {
            true => return true,
            false => return false,
        }
    }
}
