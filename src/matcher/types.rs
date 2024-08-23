use std::fmt::format;

#[derive(Debug, PartialEq, Clone)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> Self {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}
