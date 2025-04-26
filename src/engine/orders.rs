use std::fmt::{Debug, Formatter};
#[derive(Debug, Clone, Copy)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Clone, Copy)]
pub struct Order {
    pub(crate) bid_or_ask: BidOrAsk,
    size: f64,
}

impl Order {
    pub(crate) fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
}

impl Debug for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.bid_or_ask, self.size)
    }
}