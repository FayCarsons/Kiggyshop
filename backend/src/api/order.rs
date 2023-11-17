use actix_web::{
    post,
    web::{self, Path},
    HttpResponse,
};
use common::{
    order::{Order, OrderFilter},
    schema::orders::{self, fulfilled},
};
use diesel::prelude::*;

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
