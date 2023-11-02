use super::{item::Item, Order};
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cart {
    pub inner: hashbrown::HashMap<Item, u64>,
}

#[rustfmt::skip]
#[cfg_attr(feature = "backend", 
  derive(Queryable, Selectable, Associations, Identifiable), 
  diesel(belongs_to(Order)), 
  diesel(table_name = crate::schema::carts))]
pub struct DbCart {
    pub id: i32,
    pub order_id: i32,
    pub item_name: String,
    pub quantity: i32,
}

#[cfg(feature = "backend")]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::carts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCart<'a> {
    pub order_id: i32,
    pub item_name: &'a str,
    pub quantity: i32,
}
