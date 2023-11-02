pub mod cart;
pub mod item;
pub mod order;
#[cfg(feature = "backend")]
pub mod schema;

use item::Item;
use order::Order;

pub use serde::{Deserialize, Serialize};
pub use serde_json::{from_slice, from_str, to_string, to_string_pretty};
pub use hashbrown::HashMap;

pub type StockMap = HashMap<i32, Item>;
pub type CartMap = HashMap<i32, u32>;
