use std::fmt::{Debug, Formatter};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Debug for Price {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.integral + self.fractional / self.scalar)
    }
}

impl Price {
    pub(crate) fn new(price: f64) -> Price {
        let scalar = 100_000;
        let integral = price as u64;
        let fractional: u64 = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integral,
            fractional,
            scalar,
        }
    }
}