use actix_web::{delete, error, get, put, web, HttpResponse, Result};
use diesel::prelude::*;
use model::{
    address, cart, order,
    schema::{addresses, carts, orders},
};

use std::{collections::HashMap, sync::Arc};

use crate::{DbConn, DbPool};

use super::stripe;

#[get("/orders/{filter}")]
pub async fn get_orders(
    pool: web::Data<DbPool>,
    filter: web::Path<order::OrderFilter>,
) -> Result<HttpResponse> {
    let filter = filter.into_inner();

    let orders = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|e| format!("Cannot connect to database: {e}"))?;

        use order::OrderFilter::*;
        let select = orders::table.select(order::TableOrder::as_select());
        match filter {
            All => select.get_results(&mut conn),
            Shipped => select
                .filter(orders::shipped.eq(true))
                .get_results(&mut conn),
            Unshipped => select
                .filter(orders::shipped.eq(false))
                .get_results(&mut conn),
        }
        .map_err(|e| format!("Cannot fetch orders from DB: {e}"))
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let json = serde_json::to_string(&orders)?;
    Ok(HttpResponse::Ok().content_type("text/json").body(json))
}

#[put("/orders/shipped")]
pub async fn order_shipped(
    pool: web::Data<DbPool>,
    order_id: web::Path<u32>,
    tracking_number: web::Bytes,
) -> Result<HttpResponse> {
    let mut conn = pool
        .into_inner()
        .get()
        .map_err(|e| error::ErrorInternalServerError(format!("Cannot connect to DB: {e}")))?;

    let id = order_id.into_inner() as i32;
    web::block(move || {
        match diesel::update(orders::table)
            .filter(orders::id.eq(id))
            .filter(orders::shipped.eq(false))
            .set((
                orders::shipped.eq(true),
                orders::tracking_number.eq(String::from_iter(
                    tracking_number.into_iter().map(char::from),
                )),
            ))
            .execute(&mut conn)
        {
            Ok(0usize) => Err("Error: Order has already been shipped/ID is invalid"),
            _ => Ok(()),
        }
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/orders/{id}")]
pub async fn delete_order(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let id = id.into_inner();

    web::block(move || {
        let mut conn = pool.get().map_err(|_| "Cannot connect to DB")?;

        diesel::delete(orders::table.filter(orders::id.eq(id)))
            .execute(&mut conn)
            .map_err(|_| "Cannot delete order")
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn insert_order(
    mut conn: DbConn,
    cart: Arc<HashMap<model::ItemId, stripe::Item>>,
    total: u32,
    name: Arc<str>,
    email: Arc<str>,
    address: Arc<address::Address>,
) -> Result<()> {
    web::block(move || -> std::result::Result<(), String> {
        let order = order::NewOrder {
            name: &name,
            total: total as i32,
            email: &email,
        };

        let order_id = diesel::insert_into(orders::table)
            .values(&order)
            .returning(orders::id)
            .get_result::<i32>(&mut conn)
            .map_err(|_| "Cannot insert order into DB")?;

        let new_carts = cart
            .iter()
            .map(|(item_id, item)| cart::NewCart {
                order_id,
                item_id: *item_id as i32,
                quantity: item.quantity as i32,
            })
            .collect::<Vec<cart::NewCart>>();

        diesel::insert_into(carts::table)
            .values(&new_carts)
            .execute(&mut conn)
            .map_err(|_| "Cannot insert carts into DB")?;

        let insertable = address::NewAddress::new(&address, order_id);

        diesel::insert_into(addresses::table)
            .values(insertable)
            .execute(&mut conn)
            .map_err(|e| format!("Cannot insert address into db: {e}"))?;

        Ok(())
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}
