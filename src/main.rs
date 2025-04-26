use crate::engine::{MatchingEngine, TradingPair};
use engine::limit::Limit;
use engine::order_book::OrderBook;
use engine::orders::BidOrAsk::{Ask, Bid};
use engine::orders::Order;
use engine::price::Price;

mod engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let price = Price::new(50.5);
    println!("{price:?}");

    let mut limit = Limit::new(price);
    println!("{limit:?}");

    let buy_order = Order::new(Bid, 6.0);
    let buy_order2 = Order::new(Bid, 5.0);
    let sell_order = Order::new(Ask, 2.0);

    limit.add_order(sell_order);
    println!("{limit:?}");

    let mut order_book = OrderBook::new();
    order_book.add_order(sell_order, 50.5);
    order_book.add_order(buy_order, 60.5);
    order_book.add_order(buy_order2, 60.5);
    println!("{order_book:?}");
    
    let mut engine = MatchingEngine::new();
    let trading_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(trading_pair.clone());
    let buy_order = Order::new(Bid, 6.5);
    engine.place_limit_order(&trading_pair, 10.000, buy_order)?;
    println!("{engine:?}");

    Ok(())
}
