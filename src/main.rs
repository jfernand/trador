use engine::orders::BidOrAsk::{Ask, Bid};
use std::fmt::Debug;
use engine::orders::{Order, OrderBook};
use engine::price::Price;
use crate::engine::{MatchingEngine, TradingPair};

mod engine;

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: vec![],
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

fn main() {
    let price = Price::new(50.5);
    println!("{:?} ", price);

    let mut limit = Limit::new(price);
    println!("{:?} ", limit);

    let buy_order = Order::new(Bid, 6.0);
    let buy_order2 = Order::new(Bid, 5.0);
    let sell_order = Order::new(Ask, 2.0);

    limit.add_order(sell_order);
    println!("{:?} ", limit);

    let mut order_book = OrderBook::new();
    order_book.add_order(sell_order, 50.5);
    order_book.add_order(buy_order, 60.5);
    order_book.add_order(buy_order2, 60.5);
    println!("{:?} ", order_book);
    
    let mut engine = MatchingEngine::new();
    engine.add_new_market(TradingPair::new("BTC".to_string(), "USD".to_string()));
    println!("{:?}", engine)
}
