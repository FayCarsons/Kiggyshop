
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonOrder {
    pub name: String,
    pub street: String,
    pub zipcode: i32,
    pub fulfilled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Queryable, Selectable, Identifiable), diesel(table_name = crate::schema::orders), diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Order {
    #[cfg(feature = "backend")]
    pub id: i32,
    pub name: String,
    pub street: String,
    pub zipcode: i32,
    pub fulfilled: bool,
}

#[cfg(feature = "backend")]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewOrder<'a> {
    pub name: &'a str,
    pub street: &'a str,
    pub zipcode: i32,
    pub fulfilled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy)]
pub enum OrderFilter {
    All, 
    Fulfilled,
    Unfulfilled
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

