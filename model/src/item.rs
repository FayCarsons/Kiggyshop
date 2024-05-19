use serde::{Deserialize, Serialize};

use diesel::prelude::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Kind {
    BigPrint = 0,
    SmallPrint = 1,
    Button = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct InputItem {
    pub title: String,
    pub kind: String,
    pub description: String,
    pub quantity: i32,
}

impl From<Item> for InputItem {
    fn from(
        Item {
            title,
            kind,
            description,
            quantity,
            ..
        }: Item,
    ) -> Self {
        Self {
            title,
            kind,
            description,
            quantity,
        }
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Queryable, Selectable,
)]
#[diesel(table_name = crate::schema::stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
        /*
        use Kind::*;
        match self.kind {
            BigPrint => 20,
            SmallPrint => 7,
            Button => 3,
        }
        */
        match self.kind.as_str() {
            "BigPrint" => 20,
            "SmallPrint" => 7,
            "Button" => 3,
            _ => 20,
        }
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub kind: &'a str,
    pub description: &'a str,
    pub quantity: i32,
}
