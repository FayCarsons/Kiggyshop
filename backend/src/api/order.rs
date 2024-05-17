use actix_web::{
    delete, error, get, put,
    web::{self, Path},
    HttpResponse, Result,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use model::{
    cart::NewCart,
    order::{NewOrder, Order, OrderFilter},
    CartMap,
};
use r2d2::PooledConnection;

use crate::DbPool;

#[get("/orders/{filter}")]
pub async fn get_orders(
    pool: web::Data<DbPool>,
    filter: Path<OrderFilter>,
) -> Result<HttpResponse> {
    use model::schema::orders;
    let filter = filter.into_inner();

    let orders = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|e| format!("Cannot connect to database: {e}"))?;

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
        .map_err(|e| format!("Cannot fetch orders from DB: {e}"))
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let json = serde_json::to_string(&orders)?;
    Ok(HttpResponse::Ok().content_type("text/json").body(json))
}

#[put("/orders/fulfilled")]
pub async fn orders_fulfilled(
    pool: web::Data<DbPool>,
    fulfilled_orders: web::Json<Vec<Order>>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(format!("Cannot connect to database: {e}")))?;
    let ids = fulfilled_orders
        .into_inner()
        .into_iter()
        .map(|Order { id, .. }| id)
        .collect::<Vec<i32>>();
    use model::schema::orders;

    web::block(move || {
        diesel::update(orders::table.filter(orders::id.eq_any(ids)))
            .set(orders::fulfilled.eq(true))
            .execute(&mut conn)
    })
    .await
    .map(|e| match e {
        Ok(_) => Ok(()),
        Err(err) => Err(error::ErrorInternalServerError(format!(
            "Cannot update DB: {err}"
        ))),
    })??;

    Ok(HttpResponse::Ok().finish())
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

pub async fn insert_order(
    mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
    cart: CartMap,
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
                item_id: item_id.clone() as i32,
                quantity: quantity.clone() as i32,
            })
            .collect::<Vec<NewCart>>();

        diesel::insert_into(carts::table)
            .values(&new_carts)
            .execute(&mut conn)
            .map_err(|_| "Cannot insert carts into DB")?;
        Ok(inserted_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}
