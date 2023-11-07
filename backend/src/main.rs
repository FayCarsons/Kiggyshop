#![feature(once_cell_try)]
#![feature(async_closure)]

mod admin;
mod api;
mod env;
mod error;
mod stripe;

use env::Env;
use std::sync::OnceLock;

use admin::{dashboard, get_style, login, try_login, unauthorized};
use api::{
    order::get_orders,
    stock::{get_stock, init_stock},
};
use error::BackendError;

use actix_files::Files;

use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use diesel::{r2d2, SqliteConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

const BIND: (&str, u16) = ("localhost", 8081);
static ENV: OnceLock<Env> = OnceLock::new();

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    ENV.get_or_try_init(|| -> Result<Env, BackendError> {
        let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY")?;
        let init_db = std::env::var("INIT_DB")?
            .parse::<bool>()
            .map_err(|e| BackendError::EnvError(e.to_string()))?;
        let admin_pass = std::env::var("ADMIN_PASS")?;
        let database_url = std::env::var("DATABASE_URL")?;
        let completion_redirect = std::env::var("COMPLETION_REDIRECT")?;
        Ok(Env {
            init_db,
            admin_pass,
            database_url,
            stripe_secret_key,
            completion_redirect
        })
    })?;

    let env = ENV.get().cloned().unwrap_or_default();

    if env.init_db {
        init_stock()?;
    }

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(
        env.database_url
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("INVALID DB URL // DB POOL CANNOT BE BUILT");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Compress::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/admin")
                    .service(login)
                    .service(try_login)
                    .service(dashboard)
                    .service(unauthorized)
                    .service(get_style),
            )
            .service(
                web::scope("/api")
                    .service(get_stock)
                    .service(get_orders)
                    .service(
                        Files::new("/resources/images/", "./resources/images").show_files_listing(),
                    )
                    .service(
                        Files::new("/resources/fonts/", "./resources/fonts/").show_files_listing(),
                    ),
            )
    })
    .bind(BIND)?
    .run()
    .await
}
