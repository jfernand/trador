use crate::engine::orders::Order;
use crate::engine::price::Price;

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
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
}