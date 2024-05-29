use model::{
    item::{Item, NewItem, TableItem},
    schema::stock,
    CartMap, ItemId,
};

use actix_web::{
    delete, error, get, put,
    web::{self, Path},
    HttpResponse, Result,
};
use serde_json::to_string;
use std::collections::HashMap;

use diesel::{prelude::*, r2d2::ConnectionManager};

use crate::DbPool;

pub async fn item_from_db(item_id: ItemId, pool: &web::Data<DbPool>) -> Result<Item> {
    use model::schema::stock::id;

    let mut conn = pool.get().unwrap();
    web::block(move || -> std::result::Result<Item, String> {
        match stock::table
            .filter(id.eq(item_id as i32))
            .select(TableItem::as_select())
            .get_result(&mut conn)
        {
            Ok(item) => Ok(Item::from(item)),
            Err(e) => Err(format!("Cannot fetch item: {e}")),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

#[get("/stock/{item_id}")]
pub async fn get_item(item_id: Path<u32>, pool: web::Data<DbPool>) -> Result<web::Json<Item>> {
    let item_id = item_id.into_inner();
    let item: Item = item_from_db(item_id, &pool).await?;

    Ok(web::Json(item))
}

#[get("/stock")]
pub async fn get_stock(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let stock = web::block(move || {
        let mut conn = pool.get().map_err(|_| "couldn't get db connection")?;
        stock::table
            .select(TableItem::as_select())
            .get_results::<TableItem>(&mut conn)
            .map_err(|e| format!("Cannot fetch stock: {e}"))
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let stock = stock
        .into_iter()
        .map(|item| (item.id as u32, Item::from(item)))
        .collect::<HashMap<u32, Item>>();
    let ser = to_string(&stock)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(ser))
}

#[put("/stock")]
pub async fn put_item(pool: web::Data<DbPool>, item: web::Json<Item>) -> Result<HttpResponse> {
    let item = item.into_inner();

    web::block(move || {
        let item = NewItem {
            title: &item.title,
            kind: item.kind as i32,
            description: &item.description,
            quantity: item.quantity as i32,
        };

        let mut conn = pool.get().map_err(|_| "Cannot connect to DB")?;
        match diesel::insert_into(stock::table)
            .values(item)
            .execute(&mut conn)
        {
            Ok(_) => Ok(()),
            Err(_) => Err("Cannot insert item into stock table"),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[put("/stock/{item_id}")]
pub async fn update_item(
    item_id: Path<i32>,
    new_fields: web::Json<Item>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    use model::schema::stock::id;

    let item_id = item_id.into_inner();
    let Item {
        title,
        kind,
        description,
        quantity,
    } = new_fields.into_inner();

    web::block(move || {
        let new_item = NewItem {
            title: &title,
            kind: kind as i32,
            description: &description,
            quantity: quantity as i32,
        };

        let mut conn = pool.get().map_err(|_| "Cannot connect to DB".to_string())?;
        match diesel::update(stock::dsl::stock)
            .filter(id.eq(item_id))
            .set(new_item)
            .execute(&mut conn)
            .map(|_| ())
        {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Cannot update item {item_id}")),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/stock")]
pub async fn delete_items(
    pool: web::Data<DbPool>,
    item_ids: web::Json<Vec<i32>>,
) -> Result<HttpResponse> {
    use model::schema::stock::id;
    let item_ids = item_ids.into_inner();

    web::block(move || {
        let mut conn = pool.get().map_err(|_| "Cannot connect to DB")?;
        match diesel::delete(stock::table.filter(id.eq_any(item_ids))).execute(&mut conn) {
            Ok(_) => Ok(()),
            Err(_) => Err("Cannot delete items from DB"),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn dec_items(
    cart: CartMap,
    mut conn: r2d2::PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<()> {
    use model::schema::stock::{id, quantity};

    let cart = cart.clone();
    web::block(move || {
        for (item_id, qty) in cart {
            println!("Item: {item_id} {qty}");

            diesel::update(stock::table.filter(id.eq(item_id as i32)))
                .set(quantity.eq(quantity - qty as i32))
                .execute(&mut conn)
                .map_err(|_| "Cannot dec item stock in DB")?;
        }
        Ok::<(), &str>(())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}
