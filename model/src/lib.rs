pub mod cart;
pub mod item;
pub mod order;
pub mod schema;

/// Represents ID of an Item -> convert to i32 before entry into DB
pub type ItemId = u32;
/// Represents either the quantity of an item in stock, or the quantity of an item in a user's cart
pub type Quantity = u32;
