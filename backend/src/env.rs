#[derive(Clone, Debug)]
pub struct Env {
    pub init_db: bool,
    pub database_url: String,
    pub admin_pass: String,
    pub stripe_secret_key: String,
    pub completion_redirect: String
}

impl Default for Env {
    fn default() -> Self {
        Env {
            init_db: false,
            ..Default::default()
        }
    }
}