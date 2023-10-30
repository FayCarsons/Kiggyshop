use serde::{Deserialize, Serialize};
use yew::Properties;

#[cfg(feature = "backend")]
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "frontend", derive(Properties))]
#[cfg_attr(feature = "backend", derive(Queryable, Selectable), diesel(table_name = crate::schema::stock), diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Item {
    pub title: String,
    pub kind: String,
    pub description: String,
    pub quantity: i32,
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
