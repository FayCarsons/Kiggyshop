mod api;
use actix_files::Files;
use api::{order::get_orders, stock::get_stock};
pub mod error;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(
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
    .bind(("localhost", 8081))?
    .run()
    .await
}
