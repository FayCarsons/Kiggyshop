use model::{
    item::{self, Item, NewItem, NewQuantity, TableItem},
    schema::{self, stock},
    CartMap, ItemId,
};

use actix_web::{
    delete, error, get, put,
    web::{self, Path},
    HttpResponse, Result,
};
use serde_json::to_string;
use std::{collections::HashMap, sync::Arc};

use diesel::{prelude::*, r2d2::ConnectionManager};

use crate::DbPool;

use super::stripe::StripeItem;

pub async fn item_from_db(item_id: ItemId, pool: &web::Data<DbPool>) -> Result<Item> {
    let mut conn = pool.get().unwrap();
    web::block(move || -> std::result::Result<Item, String> {
        match stock::table
            .filter(stock::id.eq(item_id as i32))
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

pub async fn get_total(cart: Arc<CartMap>, pool: Arc<DbPool>) -> Result<u32> {
    let id_kind_pairs = {
        let cart = cart.clone();
        web::block(move || -> std::result::Result<Vec<(i32, i32)>, String> {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Cannot get DB connection from pool: {e}"))?;

            let ids = cart.keys().copied().map(|n| n as i32).collect::<Vec<i32>>();
            stock::table
                .select((stock::id, stock::kind))
                .filter(stock::id.eq_any(ids))
                .load::<(i32, i32)>(&mut conn)
                .map_err(|e| format!("Cannot get id-kind pairs from DB: {e}"))
        })
    }
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("<fn GET_TOTAL>\n{e}")))?;

    let id_kind_map = HashMap::<i32, i32>::from_iter(id_kind_pairs.into_iter());
    Ok(cart.iter().try_fold(0, |total, (id, qty)| {
        if let Some(kind) = id_kind_map.get(&(*id as i32)) {
            let price = match item::Kind::from(*kind) {
                item::Kind::BigPrint => 20_00,
                item::Kind::SmallPrint => 7_00,
                item::Kind::Button => 3_00,
            };
            Ok(total + price * qty)
        } else {
            Err(error::ErrorInternalServerError(
                "<fn GET_TOTAL>\nError fetching total - Item in cart not present in kind map",
            ))
        }
    })?)
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

// TODO: refactor for more effiicient, single-query batch updating
pub async fn dec_items(
    cart: Arc<HashMap<ItemId, StripeItem>>,
    mut conn: r2d2::PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<()> {
    web::block(move || {
        conn.transaction(|conn| -> diesel::QueryResult<()> {
            for (item_id, StripeItem { quantity, .. }) in cart.iter() {
                diesel::update(stock::table.filter(stock::id.eq(*item_id as i32)))
                    .set(stock::quantity.eq(stock::quantity - *quantity as i32))
                    .execute(conn)?;
            }

            Ok(())
        })
        .map_err(|e| format!("Error updating stock quantities: {e}"))
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("DB ERROR: {e}")))
}

pub async fn get_title_map(cart: Arc<CartMap>, pool: DbPool) -> Result<HashMap<ItemId, String>> {
    web::block(move || -> Result<HashMap<ItemId, String>, String> {
        let ids = cart.keys().map(|n| *n as i32).collect::<Vec<i32>>();
        let mut conn = pool
            .get()
            .map_err(|e| format!("Cannot get DB connection: {e}"))?;
        let id_title_pairs: Vec<(i32, String)> = stock::table
            .select((stock::id, stock::title))
            .filter(stock::id.eq_any(ids))
            .load::<(i32, String)>(&mut conn)
            .map_err(|e| format!("Cannot fetch titles from DB: {e}"))?;
        let id_title_map = id_title_pairs
            .into_iter()
            .map(|(id, title)| (id as u32, title))
            .collect::<HashMap<ItemId, String>>();
        Ok(id_title_map)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))
}

pub async fn get_matching_ids(ids: Vec<u32>, pool: Arc<DbPool>) -> Result<Vec<TableItem>> {
    web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|e| format!("Cannot connect to DB: {e}"))?;
        stock::table
            .select(TableItem::as_select())
            .filter(stock::id.eq_any(ids.iter().map(|n| *n as i32)))
            .get_results(&mut conn)
            .map_err(|e| format!("Cannot fetch items from DB: {e}"))
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}
