use super::cart::JsonCart;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonOrder {
    pub name: String,
    pub street: String,
    pub zipcode: String,
    pub total: i32,
    pub cart: Vec<JsonCart>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable,
)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Order {
    pub id: i32,
    pub name: String,
    pub street: String,
    pub zipcode: String,
    pub fulfilled: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewOrder<'a> {
    pub name: &'a str,
    pub street: &'a str,
    pub zipcode: &'a str,
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
