pub mod cart;
pub mod item;
pub mod order;

use order::Order;

pub use serde::{Deserialize, Serialize};
pub use serde_json::{
    error::Error as SerdeError, from_slice, from_str, to_string, to_string_pretty,
};

/// Represents ID of an Item -> convert to i32 before entry into DB
pub type ItemId = u32;
/// Represents either the quantity of an item in stock, or the quantity of an item in a user's cart
pub type Quantity = u32;
