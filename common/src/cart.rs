use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CartItem {
    item: String,
    quantity: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Cart {
    items: Vec<CartItem>,
}

impl Cart {
    pub fn new() -> Self {
        Cart { ..Self::default() }
    }
}

impl Default for Cart {
    fn default() -> Self {
        Cart { items: vec![] }
    }
}
