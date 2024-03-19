#![feature(once_cell_try)]
mod api;
mod env;
#[cfg(test)]
mod tests;
mod utils;

use api::{
    order::{delete_order, get_orders, post_order},
    stock::{delete_items, get_item, get_stock, init_stock, put_item, update_item},
    stripe::{checkout, webhook_handler},
};

use env::{init_env, Env};

use std::sync::OnceLock;

use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    SessionMiddleware,
};

use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web, App, HttpServer,
};

use diesel::{r2d2, SqliteConnection};
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

static ENV: OnceLock<Env> = OnceLock::new();

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
        .cookie_name(String::from("kiggyshop"))
        .cookie_secure(true)
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().expect("Cannot find .env");
    env_logger::init();

    let port = std::env::var("BACKEND_PORT")
        .map_err(|e| e.to_string())
        .and_then(|s| str::parse::<u16>(&s).map_err(|e| e.to_string()))
        .expect("BACKEND_PORT either not present or not valid");
    let bind = ("0.0.0.0", port);

    init_env().unwrap();
    let env = ENV.get().cloned().unwrap();

    // I think this should be something I only need in dev, so we can panic here
    if env.init_db {
        init_stock().expect("Couldn't initialize DB with stock");
    }

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(env.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("INVALID DB URL // DB POOL CANNOT BE BUILT");

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .wrap(session_middleware())
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
                    .service(delete_items)
                    .service(checkout)
                    .service(webhook_handler),
            )
            .service(webhook_handler)
    })
    .bind(bind)?
    .run()
    .await
}
