use super::Order;

use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use diesel::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Queryable, Selectable, Associations, Identifiable), diesel(belongs_to(Order)), diesel(table_name = crate::schema::carts))]
pub struct Cart {
    #[cfg(feature = "backend")]
    pub id: i32,
    #[cfg(feature = "backend")]
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
