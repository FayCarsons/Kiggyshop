pub mod cart;
pub mod item;
pub mod order;
#[cfg(feature = "backend")]
pub mod schema;
pub mod utils;

use item::Item;
use order::Order;

pub use hashbrown::HashMap;
pub use serde::{Deserialize, Serialize};
pub use serde_json::{
    error::Error as SerdeError, from_slice, from_str, to_string, to_string_pretty,
};

pub type StockMap = HashMap<i32, Item>;
pub type CartMap = HashMap<i32, u32>;
