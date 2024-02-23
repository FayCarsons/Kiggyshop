use crate::{ENV};

#[derive(Clone, Debug, Default)]
pub struct Env {
    pub init_db: bool,
    pub database_url: String,
    pub stripe_secret_key: String,
    pub completion_redirect: String,
    pub port: u16,
}

pub fn init_env() -> Result<(), String> {
    ENV.get_or_try_init(|| -> Result<Env, &str> {
        let stripe_secret_key =
            std::env::var("STRIPE_SECRET_KEY").map_err(|_| "Stripe secret not set in .env!")?;
        let init_db = std::env::var("INIT_DB")
            .unwrap_or_default()
            .parse::<bool>()
            .map_err(|_| "\'init_db\' not present in .env")?;
        let database_url =
            std::env::var("DATABASE_URL").map_err(|_| "Database URL not set in .env!")?;
        let completion_redirect = std::env::var("COMPLETION_REDIRECT")
            .map_err(|_| "Success page URL not set in .env!")?;

        let port = std::env::var("PORT")
            .map_err(|_| "Port not present in .env")
            .and_then(|s| str::parse::<u16>(&s).map_err(|_| "Cannot parse PORT from .env!"))?;
        Ok(Env {
            init_db,
            database_url,
            stripe_secret_key,
            completion_redirect,
            port,
        })
    })
    .map_err(String::from)
    .map(|_| ())
}
