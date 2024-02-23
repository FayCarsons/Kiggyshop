#![feature(once_cell_try)]
mod api;
mod env;
mod model;
mod schema;
mod stripe;
#[cfg(test)]
mod tests;
mod utils;

use actix_cors::Cors;
use api::{
    order::{delete_order, get_orders, post_order},
    stock::{delete_stock, get_item, get_stock, init_stock, put_item, update_item},
};
use env::{init_env, Env};
use stripe::{checkout, webhook_handler};

use std::sync::OnceLock;

use actix_files::Files;
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};

use diesel::{r2d2, SqliteConnection};
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

static ENV: OnceLock<Env> = OnceLock::new();

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().expect("Cannot find .env");
    env_logger::init();

    let port = std::env::var("BACKEND_PORT")
        .map_err(|e| e.to_string())
        .and_then(|s| str::parse::<u16>(&s).map_err(|e| e.to_string()))
        .expect("BACKEND_PORT either not present or not valid");
    let bind = ("localhost", port);

    init_env().unwrap();

    let env = ENV.get().cloned().unwrap_or_default();

    // I think this should be something I only need in dev, sso we can panic here
    if env.init_db {
        init_stock().expect("Couldn't initialize DB with stock");
    }

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(env.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("INVALID DB URL // DB POOL CANNOT BE BUILT");

    HttpServer::new(move || {
        let logger = Logger::default();
        let cors_cfg = Cors::default()
            .allowed_origin("localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials();

        App::new()
            .wrap(logger)
            .wrap(cors_cfg)
            .wrap(Compress::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(get_stock)
                    .service(get_orders)
                    .service(post_order)
                    .service(delete_order)
                    .service(update_item)
                    .service(get_item)
                    .service(put_item)
                    .service(delete_stock)
                    .service(checkout)
                    .service(webhook_handler)
                    .service(Files::new("/resources", "./resources").show_files_listing()),
            )
            .service(webhook_handler)
    })
    .bind(bind)?
    .run()
    .await
}
