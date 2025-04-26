use crate::engine::orders::{Order, OrderBook};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub mod orders;
pub mod price;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl Display for TradingPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
    }
}

impl Debug for TradingPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
    }
}

impl TradingPair {
    pub(crate) fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }
}

#[derive(Debug)]
pub struct MatchingEngine {
    order_books: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            order_books: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.order_books.insert(pair, OrderBook::new());
    }
    
    pub fn place_limit_order (&mut self, pair: &TradingPair, price: f64, order: Order) -> Result<(), String> {
        match self.order_books.get_mut(pair) {
            None => {
                Err(format!("An order book for {pair} does not exist"))
                    
            }
            Some(order_book) => {
                order_book.add_order(order, price);
                Ok(())
            }
        }
    }
}
