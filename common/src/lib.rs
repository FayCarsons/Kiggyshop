pub mod cart;
pub mod item;
pub mod order;

use item::Item;
use order::Order;

pub use serde::{Deserialize, Serialize};
pub use serde_json::{from_slice, from_str, to_string, to_string_pretty};
use yew::Properties;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderList {
    pub orders: Vec<Order>,
}

impl From<Vec<Order>> for OrderList {
    fn from(orders: Vec<Order>) -> Self {
        Self { orders }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Properties)]
pub struct Stock {
    pub stock: Vec<Item>,
}

impl Stock {
    pub fn empty() -> Self {
        Self { stock: vec![] }
    }
}

impl From<Vec<Item>> for Stock {
    fn from(stock: Vec<Item>) -> Self {
        Self { stock }
    }
}

impl FromIterator<Item> for Stock {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut stock = vec![];

        for i in iter {
            stock.push(i);
        }

        Self { stock }
    }
}
