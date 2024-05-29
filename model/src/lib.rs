pub mod address;
pub mod cart;
pub mod item;
pub mod order;
pub mod schema;

// Types follow a pattern of:
// {Name} -> struct for business logic, uses ideal types and is therefore safer
//  Table{Name} -> struct received from database, uses SQL-friendly types :/
//  New{Name} -> struct that is inserted into database

/// Represents ID of an Item -> convert to i32 before entry into DB
pub type ItemId = u32;
/// Represents either the quantity of an item in stock, or the quantity of an item in a user's cart
pub type Quantity = u32;

pub type CartMap = std::collections::HashMap<ItemId, Quantity>;
