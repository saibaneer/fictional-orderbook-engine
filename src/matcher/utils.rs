#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Price {
    integer: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    pub fn new(price: f64) -> Self {
        let scalar = 100000;
        let integer = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integer,
            fractional,
            scalar,
        }
    }
}
