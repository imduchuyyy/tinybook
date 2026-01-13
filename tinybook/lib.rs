pub struct TinyBook {
    pub symbol: String,

    /// bid side price levels (buy order)
    pub bids: Vec<(f64, f64)>, // (price, quantity)
}

impl TinyBook {
    pub fn new(symbol: &str) -> Self {
        TinyBook {
            symbol: symbol.to_string(),
            bids: Vec::new(),
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}
