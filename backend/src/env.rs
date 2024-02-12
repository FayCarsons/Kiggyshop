use crate::{error::ShopResult, ENV};

#[derive(Clone, Debug, Default)]
pub struct Env {
    pub init_db: bool,
    pub database_url: String,
    pub admin_pass: String,
    pub stripe_secret_key: String,
    pub completion_redirect: String,
}

pub fn init_env() -> ShopResult<()> {
    ENV.get_or_init(|| {
        let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Stripe secret not set!");
        let init_db = std::env::var("INIT_DB")
            .unwrap_or_default()
            .parse::<bool>()
            .unwrap();
        let admin_pass = std::env::var("ADMIN_PASS").expect("Admin pass not set!");
        let database_url = std::env::var("DATABASE_URL").expect("Database URL not set!");
        let completion_redirect =
            std::env::var("COMPLETION_REDIRECT").expect("Success page URL not set!");
        Env {
            init_db,
            admin_pass,
            database_url,
            stripe_secret_key,
            completion_redirect,
        }
    });
    Ok(())
}
