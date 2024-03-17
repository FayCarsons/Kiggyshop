use actix_web::{
    delete,
    error::{self, ErrorInternalServerError},
    get, post,
    web::{self, Path},
    HttpResponse, Result,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use model::{
    cart::NewCart,
    order::{JsonOrder, NewOrder, Order, OrderFilter},
};
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};

use crate::DbPool;

#[get("/orders/{filter}")]
pub async fn get_orders(
    pool: web::Data<DbPool>,
    filter: Path<OrderFilter>,
) -> Result<HttpResponse> {
    use model::schema::orders;
    let filter = filter.into_inner();

    let orders = web::block(move || {
        let mut conn = pool.get().map_err(|_| "Cannot get DB connection")?;

        match filter {
            OrderFilter::All => orders::table
                .select(Order::as_select())
                .get_results(&mut conn),
            OrderFilter::Fulfilled => orders::table
                .select(Order::as_select())
                .filter(orders::fulfilled.eq(true))
                .get_results(&mut conn),
            OrderFilter::Unfulfilled => orders::table
                .select(Order::as_select())
                .filter(orders::fulfilled.eq(false))
                .get_results(&mut conn),
        }
        .map_err(|_| "Cannot fetch orders from DB")
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let json = serde_json::to_string(&orders)?;
    Ok(HttpResponse::Ok().content_type("text/json").body(json))
}

#[delete("/orders/{id}")]
pub async fn delete_order(pool: web::Data<DbPool>, id: Path<i32>) -> Result<HttpResponse> {
    use model::schema::orders;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderId {
    pub id: i32,
}

#[post("/orders")]
pub async fn post_order(
    pool: web::Data<DbPool>,
    body: web::Json<JsonOrder>,
) -> Result<HttpResponse> {
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
    let conn = pool
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
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
) -> Result<i32> {
    web::block(move || -> std::result::Result<i32, &str> {
        use model::schema::{carts, orders};

        let order = NewOrder {
            name: &name,
            street: &street,
            zipcode: &zipcode,
            fulfilled: false,
        };

        let inserted_id = diesel::insert_into(orders::table)
            .values(&order)
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn)
            .map_err(|_| "Cannot insert order into DB")?;

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
            .execute(&mut conn)
            .map_err(|_| "Cannot insert carts into DB")?;
        Ok(inserted_id)
    })
    .await?
    .map_err(ErrorInternalServerError)
}
