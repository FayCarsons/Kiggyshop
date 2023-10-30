mod admin;
mod api;
pub mod error;
use admin::{dashboard, login, try_login, unauthorized, get_style};
use api::{
    order::get_orders,
    stock::{get_stock, init_stock},
};
use error::BackendError;

use actix_files::Files;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{r2d2, SqliteConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

const BIND: (&str, u16) = ("localhost", 8081);

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("DATABASE_URL", "../data.sqlite");
    env_logger::init();

    if std::env::var("INIT_DB").unwrap() == "TRUE" {
        init_stock().unwrap();
    }

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(
        std::env::var("DATABASE_URL").map_err(|e| BackendError::from(e))?,
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("INVALID DB URL // DB POOL CANNOT BE BUILT");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(pool.clone()))
            .service(login)
            .service(try_login)
            .service(dashboard)
            .service(unauthorized)
            .service(get_style)
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
