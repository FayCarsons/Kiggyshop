use serde::{Deserialize, Serialize};
use yew::{AttrValue, Properties};

#[cfg(feature = "backend")]
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct InputItem {
    pub title: String,
    pub kind: String,
    pub description: String,
    pub quantity: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "frontend", derive(Properties))]
#[rustfmt::skip]
#[cfg_attr(feature = "backend", 
  derive(Queryable, Selectable), 
  diesel(table_name = crate::schema::stock), 
  diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub kind: String,
    pub description: String,
    pub quantity: i32,
}

impl Item {
    pub fn price(&self) -> i64 {
        match self.kind.as_str() {
            "SmallPrint" => 7,
            "BigPrint" => 20,
            "Button" => 3,
            _ => 0,
        }
    }
}

#[cfg(feature = "backend")]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub kind: &'a str,
    pub description: &'a str,
    pub quantity: i32,
}
