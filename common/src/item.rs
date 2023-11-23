use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use diesel::prelude::*;
use yew::AttrValue;

/// Frontend version of the 'Item' struct, optimized for Yew
#[derive(Clone, Debug, PartialEq, Default)]
pub struct FrontEndItem {
    pub id: u32,
    pub title: AttrValue,
    pub kind: AttrValue,
    pub description: AttrValue,
    pub stock: u32,
}

impl From<&Item> for FrontEndItem {
    fn from(
        Item {
            id,
            title,
            kind,
            description,
            quantity,
        }: &Item,
    ) -> Self {
        Self {
            id: *id as u32,
            title: AttrValue::from(title.clone()),
            kind: AttrValue::from(kind.clone()),
            description: AttrValue::from(description.clone()),
            stock: *quantity as u32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct InputItem {
    pub title: String,
    pub kind: String,
    pub description: String,
    pub quantity: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
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
    /// Must be named quantity to prevent SQL naming issues, represents # of item in stock
    pub quantity: i32,
}

impl Item {
    #[must_use]
    pub fn price(&self) -> i64 {
        match self.kind.as_str() {
            "SmallPrint" => 7,
            "BigPrint" => 20,
            "Button" => 3,
            _ => 0,
        }
    }
}

impl From<&FrontEndItem> for Item {
    fn from(
        FrontEndItem {
            id,
            title,
            kind,
            description,
            stock,
        }: &FrontEndItem,
    ) -> Self {
        Self {
            id: *id as i32,
            title: title.to_string(),
            kind: kind.to_string(),
            description: description.to_string(),
            quantity: *stock as i32,
        }
    }
}

#[cfg(feature = "backend")]
#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub kind: &'a str,
    pub description: &'a str,
    pub quantity: i32,
}
