pub mod api;
mod env;
pub mod mail;
#[cfg(test)]
mod tests;
mod utils;

use crate::api::{
    order::{delete_order, get_orders, orders_fulfilled},
    stock::{delete_items, get_item, get_stock, put_item, update_item},
    stripe::{checkout, webhook},
};

use env::Env;

use actix_web::{middleware::Logger, web, App, HttpServer};

use diesel::{r2d2, SqliteConnection};
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;

const ADDRESS_PORT: (&str, u16) = ("0.0.0.0", 3000);

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let env = Env::new().expect("ENV ERROR: ");

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(env.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("INVALID DB URL // DB POOL CANNOT BE BUILT");

    let (mail_user, mail_pass) = {
        #[cfg(any(debug_assertions, test))]
        {
            (
                dotenvy_macro::dotenv!("MAIL_USER"),
                dotenvy_macro::dotenv!("MAIL_PASS"),
            )
        }

        #[cfg(not(any(debug_assertions, test)))]
        {
            (std::env!("MAIL_USER"), std::env!("MAIL_PASS"))
        }
    };

    let creds = Credentials::new(mail_user.to_string(), mail_pass.to_string());
    let mailer: Mailer = Mailer::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(env.clone()))
            .app_data(web::Data::new(mailer.clone()))
            .service(
                web::scope("/api")
                    .service(get_stock)
                    .service(get_orders)
                    .service(orders_fulfilled)
                    .service(delete_order)
                    .service(update_item)
                    .service(get_item)
                    .service(put_item)
                    .service(delete_items)
                    .service(checkout),
            )
            .service(webhook)
    })
    .bind(ADDRESS_PORT)?
    .run()
    .await
}
