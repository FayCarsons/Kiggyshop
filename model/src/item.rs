use serde::{Deserialize, Serialize};

use diesel::prelude::*;

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
pub enum Kind {
    #[default]
    BigPrint = 0,
    SmallPrint = 1,
    Button = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Item {
    pub title: String,
    pub kind: Kind,
    pub description: String,
    pub quantity: i32,
}

impl<'a, 'b: 'a> From<&'b Item> for NewItem<'a> {
    fn from(
        Item {
            title,
            kind,
            description,
            quantity,
        }: &'b Item,
    ) -> Self {
        NewItem {
            title,
            kind: *kind as i32,
            description,
            quantity: *quantity,
        }
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Queryable, Selectable,
)]
#[diesel(table_name = crate::schema::stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TableItem {
    pub id: i32,
    pub title: String,
    pub kind: i32,
    pub description: String,
    /// Must be named quantity to prevent SQL naming issues, represents # of item in stock
    pub quantity: i32,
}

impl Item {
    #[must_use]
    pub fn price(&self) -> i64 {
        use Kind::*;
        match self.kind {
            BigPrint => 20,
            SmallPrint => 7,
            Button => 3,
        }
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub kind: i32,
    pub description: &'a str,
    pub quantity: i32,
}
