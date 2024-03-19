#[derive(Clone, Debug, Default)]
pub struct Env {
    pub init_db: bool,
    pub database_url: String,
    pub stripe_secret_key: String,
    pub completion_redirect: String,
}

impl Env {
    pub fn new() -> Option<Env> {
        let mut vars = std::env::vars();

        let init_db = vars
            .find(|(k, _)| k == "INIT_DB")
            .and_then(|(_, v)| str::parse::<bool>(&v).ok())?;
        let database_url = vars.find(|(k, _)| k == "DATABASE_URL")?.1;
        let stripe_secret_key = vars.find(|(k, _)| k == "STRIPE_SECRET_KEY")?.1;
        let completion_redirect = vars.find(|(k, _)| k == "COMPLETION_REDIRECT")?.1;

        Some(Self {
            init_db,
            database_url,
            stripe_secret_key,
            completion_redirect,
        })
    }
}
