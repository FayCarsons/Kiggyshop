use serde::{Deserialize, Serialize};

use crate::cart::Cart;

type Name = (String, String);
// Assumes orders are US only
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Order {
    pub name: Name,
    pub street: String,
    pub zipcode: u32,
    pub cart: Cart,
    pub fulfilled: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OrderHistory {
    pub orders: Vec<Order>,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{txt}")
    }
}
