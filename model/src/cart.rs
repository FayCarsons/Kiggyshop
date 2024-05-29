use crate::{CartMap, ItemId, Quantity};

use super::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cart {
    item: ItemId,
    qty: Quantity,
}

pub fn to_map(_self: Vec<Cart>) -> std::collections::HashMap<ItemId, Quantity> {
    _self
        .into_iter()
        .map(|Cart { item, qty }| (item, qty))
        .collect()
}

impl<T: From<u32>> From<Cart> for (T, T) {
    fn from(Cart { item, qty }: Cart) -> Self {
        (item.into(), qty.into())
    }
}

#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(crate::order::TableOrder, foreign_key = order_id))]
#[diesel(belongs_to(crate::item::TableItem, foreign_key = item_id))]
#[diesel(table_name = schema::carts)]
pub struct TableCart {
    pub id: i32,
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Insertable, Clone, Copy, Debug, Serialize)]
#[diesel(table_name = crate::schema::carts)]
pub struct NewCart {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}
