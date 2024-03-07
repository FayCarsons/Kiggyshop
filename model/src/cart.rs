use super::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonCart {
    pub item: i32,
    pub qty: i32,
}

impl From<JsonCart> for (i32, i32) {
    fn from(JsonCart { item, qty }: JsonCart) -> Self {
        (item, qty)
    }
}

#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(crate::order::Order))]
#[diesel(table_name = schema::carts)]
pub struct DbCart {
    pub id: i32,
    pub order_id: i32,
    pub item_id: String,
    pub quantity: i32,
}

#[derive(Insertable, Clone, Deserialize)]
#[diesel(table_name = crate::schema::carts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCart {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}
