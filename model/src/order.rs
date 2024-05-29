use super::{cart::NewCart, CartMap};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub name: String,
    pub email: String,
    pub total: u32,
    pub cart: CartMap,
    pub shipped: bool,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable,
)]
#[diesel(table_name = crate::schema::orders)]
pub struct TableOrder {
    pub id: i32,
    name: String,
    email: String,
    total: i32,
    shipped: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub total: i32,
    pub shipped: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
#[repr(u8)]
pub enum OrderFilter {
    All,
    Shipped,
    Unshipped,
}
