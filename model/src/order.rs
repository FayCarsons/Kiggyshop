use crate::address;

use super::CartMap;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub total: u32,
    pub cart: CartMap,
    pub address: address::Address,
    pub shipped: bool,
    pub tracking_number: Option<String>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable,
)]
#[diesel(table_name = crate::schema::orders)]
pub struct TableOrder {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub total: i32,
    pub shipped: bool,
    pub tracking_number: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub total: i32,
    pub shipped: bool,
}

impl<'a, 'b: 'a> From<&'b Order> for NewOrder<'a> {
    fn from(
        Order {
            name, email, total, ..
        }: &'b Order,
    ) -> Self {
        Self {
            name,
            email,
            total: *total as i32,
            shipped: false,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
#[repr(u8)]
pub enum OrderFilter {
    All = 0,
    Shipped = 1,
    Unshipped = 2,
}
