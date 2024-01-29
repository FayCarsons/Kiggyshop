use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use super::item::Item;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckoutCart {
    pub inner: HashMap<Item, u32>,
}
/* 
impl CheckoutCart {
    pub fn from_ctx(stock: &StockMap, cart: &CartMap) -> Self {
        let res = cart
            .iter()
            .map(|(id, qty)| {
                let item = stock.get(id).unwrap();
                let item = Item::from(item);

                (item.clone(), *qty)
            })
            .collect::<HashMap<Item, u32>>();

        Self { inner: res }
    }
}
*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonOrder {
    pub name: String,
    pub street: String,
    pub zipcode: i32,
    pub total: i32,
    pub fulfilled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = super::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Order {
    pub id: i32,
    pub name: String,
    pub street: String,
    pub zipcode: i32,
    pub fulfilled: bool,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewOrder<'a> {
    pub name: &'a str,
    pub street: &'a str,
    pub zipcode: i32,
    pub fulfilled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
pub enum OrderFilter {
    All,
    Fulfilled,
    Unfulfilled,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ Order: \n\t name: {:?}, \n street: {}, \n zipcode: {}, \n, fulfilled: {} \n }}",
            self.name, self.street, self.zipcode, self.fulfilled
        )
    }
}
