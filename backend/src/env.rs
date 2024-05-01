#[derive(Clone, Debug, Default)]
pub struct Env<'a> {
    pub database_url: &'a str,
    pub stripe_secret: &'a str,
    pub stripe_key: &'a str,
    pub completion_redirect: &'a str,
}

impl Env<'static> {
    pub fn new() -> Result<Env<'static>, String> {
        #[cfg(debug_assertions)]
        {
            // We are in development
            let database_url = dotenvy_macro::dotenv!("REMOTE_DATABASE_PATH");
            let stripe_secret = dotenvy_macro::dotenv!("STRIPE_SECRET");
            let stripe_key = dotenvy_macro::dotenv!("STRIPE_KEY");
            let completion_redirect = dotenvy_macro::dotenv!("COMPLETION_REDIRECT");
            Ok(Self {
                database_url,
                stripe_secret,
                stripe_key,
                completion_redirect,
            })
        }
        #[cfg(not(debug_assertions))]
        {
            // Building in Docker container
            let database_url = std::env!("REMOTE_DATABASE_PATH");
            let stripe_secret = std::env!("STRIPE_SECRET");
            let stripe_key = std::env!("STRIPE_KEY");
            let completion_redirect = std::env!("COMPLETION_REDIRECT");
            Ok(Self {
                database_url,
                stripe_secret,
                stripe_key,
                completion_redirect,
            })
        }
    }
}
