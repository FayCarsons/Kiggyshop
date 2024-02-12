mod admin;
mod api;
mod env;
mod error;
mod model;
mod schema;
mod stripe;
#[cfg(test)]
mod tests;
mod utils;

use actix_cors::Cors;
use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use admin::{
    get_admin_dashboard, get_dashboard, get_js, get_style, login, post_admin_dashboard,
    post_dashboard, try_login, upload_image,
};
use api::{
    order::{delete_order, get_orders, post_order},
    stock::{delete_stock, get_item, get_stock, init_stock, put_item, update_item},
};
use env::{init_env, Env};
use stripe::{checkout, webhook_handler};

use std::sync::OnceLock;

use actix_files::Files;
use actix_web::{
    cookie::{Key, SameSite},
    http::header,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};

use diesel::{r2d2, SqliteConnection};
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

static ENV: OnceLock<Env> = OnceLock::new();

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name(String::from("admin-password"))
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
    let bind = ("localhost", port);

    init_env()?;

    let env = ENV.get().cloned().unwrap_or_default();

    if env.init_db {
        init_stock()?;
    }

    // DATABASE POOL BUILDING
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(env.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("INVALID DB URL // DB POOL CANNOT BE BUILT");

    HttpServer::new(move || {
        let logger = Logger::default();
        let _cors_cfg = Cors::default()
            .allowed_origin("localhost:8080")
            .allowed_origin("127.0.0.1:8081")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials();

        App::new()
            .wrap(logger)
            //.wrap(cors_cfg)
            .wrap(Compress::default())
            .wrap(session_middleware())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/admin")
                    .service(login)
                    .service(try_login)
                    .service(get_dashboard)
                    .service(post_dashboard)
                    .service(get_admin_dashboard)
                    .service(post_admin_dashboard)
                    .service(get_style)
                    .service(get_js)
                    .service(upload_image),
            )
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
