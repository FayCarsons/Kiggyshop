use crate::{
    error::{BackendError, ShopResult},
    ENV,
};

#[derive(Clone, Debug, Default)]
pub struct Env {
    pub init_db: bool,
    pub database_url: String,
    pub admin_pass: String,
    pub stripe_secret_key: String,
    pub completion_redirect: String,
}

pub fn init_env() -> ShopResult<()> {
    ENV.get_or_try_init(|| -> ShopResult<Env> {
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
            completion_redirect,
        })
    })?;
    Ok(())
}
