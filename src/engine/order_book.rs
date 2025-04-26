use crate::engine::limit::Limit;
use crate::engine::orders::{BidOrAsk, Order};
use crate::engine::price::Price;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct OrderBook {
    bids: HashMap<Price, Limit>,
    asks: HashMap<Price, Limit>,
}

impl OrderBook {
    pub(crate) fn new() -> OrderBook {
        OrderBook::default()
    }

    pub(crate) fn add_order(&mut self, order: Order, price: f64) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::Bid => self.store_bid(order, price),
            BidOrAsk::Ask => self.store_ask(order, price),
        }
    }

    fn store_bid(&mut self, order: Order, price: Price) {
        let limit = self.bids.get_mut(&price);
        match limit {
            None => {
                let mut limit = Limit::new(price);
                limit.add_order(order);
                self.bids.insert(price, limit);
            }
            Some(limit) => {
                limit.add_order(order);
            }
        }
    }

    fn store_ask(&mut self, order: Order, price: Price) {
        let limit = self.asks.get_mut(&price);
        match limit {
            None => {
                let mut limit = Limit::new(price);
                limit.add_order(order);
                self.asks.insert(price, limit);
            }
            Some(limit) => {
                limit.add_order(order);
            }
        }
    }
}