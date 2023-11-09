#![feature(once_cell_try)]
#![feature(async_closure)]

mod admin;
mod api;
mod env;
mod error;
mod stripe;

use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use admin::{
    get_admin_dashboard, get_dashboard, get_style, login, post_admin_dashboard, post_dashboard,
    try_login,
};
use api::{
    order::get_orders,
    stock::{delete_stock, get_item, get_stock, init_stock, put_item, update_item},
};
use env::{init_env, Env};
use error::BackendError;

use std::sync::OnceLock;

use actix_files::Files;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::{Compress, Logger},
    web, App, HttpServer,
};

use diesel::{r2d2, SqliteConnection};
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

const BIND: (&str, u16) = ("localhost", 8081);
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
    dotenv::dotenv().ok();
    env_logger::init();

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

    // RATE LIMITER CONFIG
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(25)
        .finish()
        .ok_or_else(|| BackendError::RateLimitError)?;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Compress::default())
            .wrap(session_middleware())
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/admin")
                    .service(login)
                    .service(try_login)
                    .service(get_dashboard)
                    .service(post_dashboard)
                    .service(get_admin_dashboard)
                    .service(post_admin_dashboard)
                    .service(get_style),
            )
            .service(
                web::scope("/api")
                    .service(get_stock)
                    .service(get_orders)
                    .service(update_item)
                    .service(get_item)
                    .service(put_item)
                    .service(delete_stock)
                    .service(Files::new("/resources", "./resources").show_files_listing()),
            )
    })
    .bind(BIND)?
    .run()
    .await
}
