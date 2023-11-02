use std::collections::HashMap;

use actix_web::{
    post, put,
    web::{self, Json, Path},
    HttpResponse,
};
use common::{
    cart::NewCart,
    item::Item,
    order::{NewOrder, Order, OrderFilter},
    schema::{
        carts,
        orders::{self, fulfilled},
    },
};
use diesel::prelude::*;

use crate::{error::ShopResult, DbPool};

#[put("/order/put")]
pub async fn new_order(
    pool: web::Data<DbPool>,
    order: web::Json<Order>,
) -> ShopResult<HttpResponse> {
    // STRIPE IMPLEMENTATION GOES HERE
    // FOLLOWED BY EMAILS VA SMTP TO BOTH CUSTOMER && Kristen

    web::block(move || {
        let order = order.into_inner().clone();
        let order = NewOrder {
            name: &order.name,
            street: &order.street,
            zipcode: order.zipcode,
            fulfilled: order.fulfilled,
        };
        let mut conn = pool.get().unwrap();
        let inserted_id = diesel::insert_into(orders::table)
            .values(&order)
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn)
            .unwrap();
        let new_carts = [
            NewCart {
                order_id: inserted_id,
                item_name: "cat",
                quantity: 420,
            },
            NewCart {
                order_id: inserted_id,
                item_name: "bunny",
                quantity: 69,
            },
        ];
        diesel::insert_into(carts::table)
            .values(&new_carts)
            .execute(&mut conn)
            .unwrap();
    })
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[post("/orders/get/{filter}")]
pub async fn get_orders(
    pool: web::Data<DbPool>,
    filter: Path<OrderFilter>,
) -> ShopResult<HttpResponse> {
    let filter = filter.into_inner();

    let orders: Vec<Order> = web::block(move || {
        let mut conn = pool.get().unwrap();

        match filter {
            OrderFilter::All => orders::table
                .select(Order::as_select())
                .get_results(&mut conn)
                .unwrap(),
            OrderFilter::Fulfilled => orders::table
                .select(Order::as_select())
                .filter(fulfilled.eq(true))
                .get_results(&mut conn)
                .unwrap(),
            OrderFilter::Unfulfilled => orders::table
                .select(Order::as_select())
                .filter(fulfilled.eq(false))
                .get_results(&mut conn)
                .unwrap(),
        }
    })
    .await
    .unwrap();

    let json = serde_json::to_string(&orders).unwrap();
    Ok(HttpResponse::Ok().content_type("text/json").body(json))
}

#[post("/order/total")]
pub async fn calc_total(cart: Json<HashMap<Item, u32>>) -> ShopResult<HttpResponse> {
    let total: i64 = cart.iter().map(|(k, v)| k.price() * *v as i64).sum();

    Ok(HttpResponse::Ok().body(total.to_string()))
}
