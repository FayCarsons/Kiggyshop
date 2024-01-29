use super::Order;
use diesel::prelude::*;


#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = super::schema::carts)]
pub struct DbCart {
    pub id: i32,
    pub order_id: i32,
    pub item_id: String,
    pub quantity: i32,
}


#[derive(Insertable, Clone)]
#[diesel(table_name = super::schema::carts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCart {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}
