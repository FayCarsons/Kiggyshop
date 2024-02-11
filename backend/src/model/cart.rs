use super::Order;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonCart {
    item:  i32,
    qty: i32
}

impl From<JsonCart> for (i32, i32) {
    fn from(JsonCart { item, qty }: JsonCart) -> Self {
        (item, qty)
    }
}


#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = super::schema::carts)]
pub struct DbCart {
    pub id: i32,
    pub order_id: i32,
    pub item_id: String,
    pub quantity: i32,
}


#[derive(Insertable, Clone, Deserialize)]
#[diesel(table_name = super::schema::carts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCart {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}
