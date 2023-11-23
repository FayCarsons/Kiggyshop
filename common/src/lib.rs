pub mod cart;
pub mod item;
pub mod order;
#[cfg(feature = "backend")]
pub mod schema;
pub mod utils;

use item::FrontEndItem;
use order::Order;

pub use hashbrown::HashMap;
pub use serde::{Deserialize, Serialize};
pub use serde_json::{
    error::Error as SerdeError, from_slice, from_str, to_string, to_string_pretty,
};

/// Represents ID of an Item -> convert to i32 before entry into DB
pub type ItemId = u32;
/// Represents either the quantity of an item in stock, or the quantity of an item in a user's cart
pub type Quantity = u32;

/// Global frontend context object representing all items in stock
pub type StockMap = HashMap<ItemId, FrontEndItem>;
/// Global frontend context object representing user's current cart
pub type CartMap = HashMap<ItemId, Quantity>;
