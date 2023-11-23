use actix_web::{
    post,
    web::{self, Path},
    HttpResponse,
};
use common::{
    cart::NewCart,
    order::{NewOrder, Order, OrderFilter},
    schema::orders::{self, fulfilled},
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

use crate::{error::ShopResult, DbPool};

#[post("/orders/get/{filter}")]
pub async fn get_orders(
    pool: web::Data<DbPool>,
    filter: Path<OrderFilter>,
) -> ShopResult<HttpResponse> {
    let filter = filter.into_inner();

    let orders = web::block(move || -> ShopResult<Vec<Order>> {
        let mut conn = pool.get()?;

        let res = match filter {
            OrderFilter::All => orders::table
                .select(Order::as_select())
                .get_results(&mut conn)?,
            OrderFilter::Fulfilled => orders::table
                .select(Order::as_select())
                .filter(fulfilled.eq(true))
                .get_results(&mut conn)?,
            OrderFilter::Unfulfilled => orders::table
                .select(Order::as_select())
                .filter(fulfilled.eq(false))
                .get_results(&mut conn)?,
        };
        Ok(res)
    })
    .await??;

    let json = serde_json::to_string(&orders)?;
    Ok(HttpResponse::Ok().content_type("text/json").body(json))
}

pub async fn insert_order(
    mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
    cart: Vec<(i32, i32)>,
    name: String,
    street: String,
    zipcode: i32,
) -> ShopResult<()> {
    web::block(move || -> ShopResult<()> {
        use common::schema::carts;

        let order = NewOrder {
            name: &name,
            street: &street,
            zipcode,
            fulfilled: false,
        };

        let inserted_id = diesel::insert_into(orders::table)
            .values(&order)
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn)?;

        let new_carts = cart
            .into_iter()
            .map(|(item_id, quantity)| NewCart {
                order_id: inserted_id,
                item_id,
                quantity,
            })
            .collect::<Vec<NewCart>>();

        diesel::insert_into(carts::table)
            .values(&new_carts)
            .execute(&mut conn)?;
        Ok(())
    })
    .await
    .expect("CRASHED WHILE INSERTING ORDER INTO DB")
}
