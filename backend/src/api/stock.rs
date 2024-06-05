use model::{item, schema::stock};

use actix_web::{delete, error, get, put, web, HttpResponse, Result};
use std::{collections::HashMap, sync::Arc};

use diesel::{prelude::*, r2d2::ConnectionManager};

use super::metrics::log_user;
use crate::DbPool;

use super::stripe;

pub async fn item_from_db(item_id: model::ItemId, pool: &web::Data<DbPool>) -> Result<item::Item> {
    let mut conn = pool.get().unwrap();
    web::block(move || -> std::result::Result<item::Item, String> {
        match stock::table
            .filter(stock::id.eq(item_id as i32))
            .select(item::TableItem::as_select())
            .get_result(&mut conn)
        {
            Ok(item) => Ok(item::Item::from(item)),
            Err(e) => Err(format!("Cannot fetch item: {e}")),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

pub async fn get_total(cart: Arc<model::CartMap>, pool: Arc<DbPool>) -> Result<u32> {
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
    cart.iter().try_fold(0, |total, (id, qty)| {
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
    })
}

#[get("/stock/{item_id}")]
pub async fn get_item(
    item_id: web::Path<u32>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<item::Item>> {
    let item_id = item_id.into_inner();
    let item = item_from_db(item_id, &pool).await?;

    Ok(web::Json(item))
}

#[get("/stock")]
pub async fn get_stock(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    if let Ok(metrics_conn) = pool.get() {
        actix_web::rt::spawn(log_user(req, metrics_conn));
    }

    let stock = {
        let mut stock_conn = pool
            .get()
            .map_err(|e| error::ErrorInternalServerError(format!("Cannot connect to DB: {e}")))?;

        web::block(move || {
            stock::table
                .select(item::TableItem::as_select())
                .get_results::<item::TableItem>(&mut stock_conn)
                .map_err(|e| format!("Cannot fetch stock: {e}"))
        })
        .await?
        .map_err(error::ErrorInternalServerError)?
    }
    .into_iter()
    .map(|item @ item::TableItem { id, .. }| (id as u32, item::Item::from(item)))
    .collect::<HashMap<u32, item::Item>>();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(stock))
}

#[put("/stock")]
pub async fn put_item(
    pool: web::Data<DbPool>,
    item: web::Json<item::Item>,
) -> Result<HttpResponse> {
    let item = item.into_inner();

    web::block(move || {
        let item = item::NewItem::from(&item);
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
    item_id: web::Path<i32>,
    new_fields: web::Json<item::Item>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    use model::schema::stock::id;

    let item_id = item_id.into_inner();

    let new_fields = new_fields.into_inner();
    web::block(move || {
        let new_item = item::NewItem::from(&new_fields);

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
    cart: Arc<HashMap<model::ItemId, stripe::Item>>,
    mut conn: r2d2::PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<()> {
    web::block(move || {
        conn.transaction(|conn| -> diesel::QueryResult<()> {
            for (item_id, stripe::Item { quantity, .. }) in cart.iter() {
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

pub async fn get_title_map(
    cart: Arc<model::CartMap>,
    pool: DbPool,
) -> Result<HashMap<model::ItemId, String>> {
    web::block(move || -> Result<HashMap<model::ItemId, String>, String> {
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
            .collect::<HashMap<model::ItemId, String>>();

        Ok(id_title_map)
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

pub async fn get_matching_ids(ids: Vec<u32>, pool: Arc<DbPool>) -> Result<Vec<item::TableItem>> {
    web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|e| format!("Cannot connect to DB: {e}"))?;
        stock::table
            .select(item::TableItem::as_select())
            .filter(stock::id.eq_any(ids.iter().map(|n| *n as i32)))
            .get_results(&mut conn)
            .map_err(|e| format!("Cannot fetch items from DB: {e}"))
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}
