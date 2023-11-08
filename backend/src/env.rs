#[derive(Clone, Debug, Default)]
pub struct Env {
    pub init_db: bool,
    pub database_url: String,
    pub admin_pass: String,
    pub stripe_secret_key: String,
    pub completion_redirect: String,
}
