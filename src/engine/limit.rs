use crate::engine::orders::Order;
use crate::engine::price::Price;

#[derive(Debug)]
pub struct Limit {
    price: Price,
    pub(crate) orders: Vec<Order>,
}

impl Limit {
    pub(crate) fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: vec![],
        }
    }

    pub(crate) fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub(crate) fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in &mut self.orders {
            if market_order.size >= limit_order.size {
                market_order.size -= limit_order.size;
                limit_order.size = 0.0;
            } else {
                limit_order.size -= market_order.size;
                market_order.size = 0.0;
            }
            if market_order.is_filled() {
                break;
            }
        }
    }
}
