use serde::{Deserialize, Serialize};

use diesel::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum Kind {
    #[default]
    BigPrint = 0,
    SmallPrint = 1,
    Button = 2,
}

impl From<i32> for Kind {
    fn from(value: i32) -> Self {
        unsafe { std::mem::transmute(value as u8) }
    }
}

impl From<Kind> for i32 {
    fn from(value: Kind) -> Self {
        (value as u8) as i32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Item {
    pub title: String,
    pub kind: Kind,
    pub description: String,
    pub quantity: u32,
}

impl std::hash::Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state)
    }
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
            quantity: *quantity as i32,
        }
    }
}

impl From<TableItem> for Item {
    fn from(
        TableItem {
            title,
            kind,
            description,
            quantity,
            ..
        }: TableItem,
    ) -> Self {
        Self {
            title,
            kind: Kind::from(kind),
            description,
            quantity: quantity as u32,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::stock)]
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
pub struct NewItem<'a> {
    pub title: &'a str,
    pub kind: i32,
    pub description: &'a str,
    pub quantity: i32,
}
