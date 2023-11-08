use common::{
    item::{InputItem, Item, NewItem},
    schema::stock,
    StockMap,
};

use actix_web::{delete, get, put, web, HttpResponse};
use serde_json::to_string;
use std::fs;

use diesel::prelude::*;

use crate::{
    error::{BackendError, ShopResult},
    DbPool, ENV,
};

#[get("/stock/get")]
pub async fn get_stock(pool: web::Data<DbPool>) -> ShopResult<HttpResponse> {
    let stock: Vec<Item> = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection");
        stock::table
            .select(Item::as_select())
            .get_results(&mut conn)
            .expect("CANNOT GET STOCK FROM DB")
    })
    .await
    .map_err(|e| BackendError::FileReadError(e.to_string()))?;

    let hm: StockMap = stock
        .iter()
        .map(|item| (item.id, item.clone()))
        .collect::<StockMap>();

    let ser = to_string(&hm)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(ser))
}

#[put("/stock/put")]
pub async fn put_item(pool: web::Data<DbPool>, item: web::Json<Item>) -> ShopResult<HttpResponse> {
    let item = item.into_inner();

    web::block(move || {
        let item = NewItem {
            title: &item.title,
            kind: &item.kind,
            description: &item.description,
            quantity: item.quantity,
        };

        let mut conn = pool.get().expect("CANNOT GET DB CONNECTION");
        diesel::insert_into(stock::table)
            .values(item)
            .execute(&mut conn)
            .map(|_| ())
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/stock/delete/{item_id}")]
pub async fn delete_stock(
    pool: web::Data<DbPool>,
    item_id: web::Path<i32>,
) -> ShopResult<HttpResponse> {
    use common::schema::stock::*;
    let item_id = item_id.into_inner();

    web::block(move || -> ShopResult<()> {
        let mut conn = pool.get()?;
        diesel::delete(stock::table.filter(id.eq(item_id))).execute(&mut conn)?;
        Ok(())
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

/// Only needed if DB does not have stock.
/// Requires env var `INIT_DB=TRUE` to run
pub fn init_stock() -> Result<(), BackendError> {
    let buffer = fs::read_to_string("stock.json")?;

    let de = serde_json::from_str::<Vec<InputItem>>(&buffer)?;
    let ins: Vec<NewItem> = de
        .iter()
        .map(|item| NewItem {
            title: &item.title,
            kind: &item.kind,
            description: &item.description,
            quantity: item.quantity,
        })
        .collect();

    let mut conn =
        SqliteConnection::establish(&ENV.get().cloned().unwrap_or_default().database_url)
            .map_err(|e| BackendError::DbError(e.to_string()))?;
    diesel::insert_into(stock::table)
        .values(ins)
        .execute(&mut conn)?;

    Ok(())
}
