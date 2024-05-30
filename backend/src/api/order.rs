use actix_web::{
    delete, error, get, put,
    web::{self, Path},
    HttpResponse, Result,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use model::{
    address::{Address, NewAddress},
    cart::NewCart,
    order::{NewOrder, OrderFilter, TableOrder},
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
                .select(TableOrder::as_select())
                .get_results(&mut conn),
            OrderFilter::Shipped => orders::table
                .select(TableOrder::as_select())
                .filter(orders::shipped.eq(true))
                .get_results(&mut conn),
            OrderFilter::Unshipped => orders::table
                .select(TableOrder::as_select())
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

#[put("/orders/fulfilled")]
pub async fn orders_fulfilled(
    pool: web::Data<DbPool>,
    fulfilled_orders: web::Json<Vec<TableOrder>>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(format!("Cannot connect to database: {e}")))?;
    let ids = fulfilled_orders
        .into_inner()
        .into_iter()
        .map(|TableOrder { id, .. }| id)
        .collect::<Vec<i32>>();
    use model::schema::orders;

    web::block(move || {
        diesel::update(orders::table.filter(orders::id.eq_any(ids)))
            .set(orders::shipped.eq(true))
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
    email: Option<String>,
    address: stripe::Address,
) -> Result<()> {
    web::block(move || -> std::result::Result<(), String> {
        use model::schema::{addresses, carts, orders};

        // NOTE: add actual total
        let order = NewOrder {
            name: &name,
            total: 0,
            email: &email.unwrap_or_default(),
            shipped: false,
        };

        let order_id = diesel::insert_into(orders::table)
            .values(&order)
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn)
            .map_err(|_| "Cannot insert order into DB")?;

        let new_carts = cart
            .into_iter()
            .map(|(item_id, quantity)| NewCart {
                order_id,
                item_id: item_id as i32,
                quantity: quantity as i32,
            })
            .collect::<Vec<NewCart>>();

        diesel::insert_into(carts::table)
            .values(&new_carts)
            .execute(&mut conn)
            .map_err(|_| "Cannot insert carts into DB")?;

        let (number, street) = {
            let full = address
                .line1
                .map(|line| line + &address.line2.unwrap_or_default())
                .ok_or("No address field in order")?;
            full.trim()
                .split_once(" ")
                .ok_or(format!("Malformed address: {full}"))
                .and_then(|(number, name)| match str::parse::<u32>(number) {
                    Ok(num) => Ok((num, name.to_string())),
                    Err(e) => Err(format!("Invalid house number: {e}")),
                })?
        };

        let zipcode = address
            .postal_code
            .ok_or("No zipcode specified for order")
            .and_then(|str| str::parse::<u32>(&str).map_err(|_| "Cannot parse zipcode"))?;

        let address = Address {
            number,
            street,
            city: address.city.ok_or("No city specified for order")?,
            state: address.state.ok_or("No state specified for order")?,
            zipcode,
            name,
        };

        let insertable = NewAddress::new(&address, order_id);

        diesel::insert_into(addresses::table)
            .values(insertable)
            .execute(&mut conn)
            .map_err(|e| format!("Cannot insert address into db: {e}"))?;

        Ok(())
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}
