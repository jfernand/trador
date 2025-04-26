use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Clone, Copy)]
pub struct Order {
    pub(crate) bid_or_ask: BidOrAsk,
    pub(crate) size: f64,
}

impl Order {
    pub(crate) fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
    pub(crate) fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

impl Debug for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:#.2}", self.bid_or_ask, self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::limit::Limit;
    use crate::engine::price::Price;

    #[test]
    fn limit_order_filled() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 100.0);
        limit.fill_order(&mut market_sell_order);
        println!("{market_sell_order:?}");

        println!("{limit:?}");
        assert!(market_sell_order.is_filled());
    }

    #[test]
    fn limit_order_not_filled() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);
        println!("{market_sell_order:?}");

        println!("{limit:?}");
        assert!(market_sell_order.is_filled());
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }
}