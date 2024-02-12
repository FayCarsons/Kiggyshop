use crate::model::{
    cart::NewCart,
    order::{JsonOrder, NewOrder, Order, OrderFilter},
};
use actix_web::{
    delete, post,
    web::{self, Path},
    HttpResponse,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};

use crate::{error::ShopResult, DbPool};

#[post("/orders/get/{filter}")]
pub async fn get_orders(
    pool: web::Data<DbPool>,
    filter: Path<OrderFilter>,
) -> ShopResult<HttpResponse> {
    use crate::schema::orders;
    let filter = filter.into_inner();

    let orders = web::block(move || -> ShopResult<Vec<Order>> {
        let mut conn = pool.get()?;

        let res = match filter {
            OrderFilter::All => orders::table
                .select(Order::as_select())
                .get_results(&mut conn)?,
            OrderFilter::Fulfilled => orders::table
                .select(Order::as_select())
                .filter(orders::fulfilled.eq(true))
                .get_results(&mut conn)?,
            OrderFilter::Unfulfilled => orders::table
                .select(Order::as_select())
                .filter(orders::fulfilled.eq(false))
                .get_results(&mut conn)?,
        };
        Ok(res)
    })
    .await??;

    let json = serde_json::to_string(&orders)?;
    Ok(HttpResponse::Ok().content_type("text/json").body(json))
}

#[delete("/orders/{id}")]
pub async fn delete_order(pool: web::Data<DbPool>, id: Path<i32>) -> ShopResult<HttpResponse> {
    use crate::schema::orders;
    let id = id.into_inner();
    let mut conn = pool.get()?;

    web::block(move || -> ShopResult<()> {
        match diesel::delete(orders::table.filter(orders::id.eq(id))).execute(&mut conn) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("DATABASE ERROR: {e}");
                panic!()
            }
        }
        Ok(())
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderId {
    pub id: i32,
}

#[post("/orders")]
pub async fn post_order(
    pool: web::Data<DbPool>,
    body: web::Json<JsonOrder>,
) -> ShopResult<HttpResponse> {
    let JsonOrder {
        name,
        street,
        zipcode,
        cart,
        ..
    } = body.into_inner();

    let cart = cart
        .into_iter()
        .map(<(i32, i32)>::from)
        .collect::<Vec<(i32, i32)>>();
    let conn = pool.get()?;
    let id = insert_order(conn, cart, name, street, zipcode).await?;
    let id = OrderId { id };

    Ok(HttpResponse::Ok().content_type("application/json").json(id))
}

pub async fn insert_order(
    mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
    cart: Vec<(i32, i32)>,
    name: String,
    street: String,
    zipcode: String,
) -> ShopResult<i32> {
    web::block(move || -> ShopResult<i32> {
        use crate::schema::{carts, orders};

        let order = NewOrder {
            name: &name,
            street: &street,
            zipcode: &zipcode,
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
        Ok(inserted_id)
    })
    .await
    .expect("Failure while inserting order into DB")
}
