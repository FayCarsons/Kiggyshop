use common::{
    item::{InputItem, Item, NewItem},
    schema::stock::{self, id},
    StockMap,
};

use actix_web::{
    delete, get, put,
    web::{self, Path},
    HttpResponse,
};
use serde_json::to_string;
use std::fs;

use diesel::prelude::*;

use crate::{
    error::{BackendError, ShopResult},
    DbPool, ENV,
};

pub async fn item_from_db(item_id: i32, pool: &web::Data<DbPool>) -> ShopResult<Item> {
    let mut conn = pool.get().unwrap();
    web::block(move || -> ShopResult<Item> {
        Ok(stock::table
            .filter(id.eq(item_id))
            .select(Item::as_select())
            .get_result(&mut conn)?)
    })
    .await?
}

#[get("/stock/get_single/{item_id}")]
pub async fn get_item(item_id: Path<i32>, pool: web::Data<DbPool>) -> ShopResult<web::Json<Item>> {
    let item_id = item_id.into_inner();
    let item: Item = item_from_db(item_id, &pool).await?;

    Ok(web::Json(item))
}

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

#[put("/stock/add")]
pub async fn put_item(
    pool: web::Data<DbPool>,
    item: web::Json<InputItem>,
) -> ShopResult<HttpResponse> {
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

#[put("/stock/update/{item_id}")]
pub async fn update_item(
    item_id: Path<i32>,
    new_fields: web::Json<InputItem>,
    pool: web::Data<DbPool>,
) -> ShopResult<HttpResponse> {
    let item_id = item_id.into_inner();
    let InputItem {
        title,
        kind,
        description,
        quantity,
    } = new_fields.into_inner();

    web::block(move || -> ShopResult<()> {
        let new_item = NewItem {
            title: &title,
            kind: &kind,
            description: &description,
            quantity,
        };

        let mut conn = pool.get()?;
        Ok(diesel::update(stock::dsl::stock)
            .filter(id.eq(item_id))
            .set(new_item)
            .execute(&mut conn)
            .map(|_| ())?)
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/stock/delete")]
pub async fn delete_stock(
    pool: web::Data<DbPool>,
    item_ids: web::Json<Vec<i32>>,
) -> ShopResult<HttpResponse> {
    use common::schema::stock::*;
    let item_ids = item_ids.into_inner();

    web::block(move || -> ShopResult<()> {
        let mut conn = pool.get()?;
        diesel::delete(stock::table.filter(id.eq_any(item_ids))).execute(&mut conn)?;
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
