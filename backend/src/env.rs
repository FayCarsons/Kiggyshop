#[derive(Clone, Debug, Default)]
pub struct Env {
    pub database_url: &'static str,
    pub stripe_secret: &'static str,
    pub stripe_key: &'static str,
    pub completion_redirect: &'static str,
    pub mail_user: &'static str,
    pub mail_pass: &'static str,
}

impl Env {
    pub const fn new() -> Env {
        #[cfg(any(debug_assertions, test))]
        {
            // We are in development
            let database_url = dotenvy_macro::dotenv!("REMOTE_DATABASE_PATH");
            let stripe_secret = dotenvy_macro::dotenv!("STRIPE_SECRET");
            let stripe_key = dotenvy_macro::dotenv!("STRIPE_KEY");
            let completion_redirect = dotenvy_macro::dotenv!("COMPLETION_REDIRECT");
            let mail_user = dotenvy_macro::dotenv!("MAIL_USER");
            let mail_pass = dotenvy_macro::dotenv!("MAIL_PASS");

            Self {
                database_url,
                stripe_secret,
                stripe_key,
                completion_redirect,
                mail_user,
                mail_pass,
            }
        }
        #[cfg(not(any(debug_assertions, test)))]
        {
            // Building in Docker container
            let database_url = std::env!("REMOTE_DATABASE_PATH");
            let stripe_secret = std::env!("STRIPE_SECRET");
            let stripe_key = std::env!("STRIPE_KEY");
            let completion_redirect = std::env!("COMPLETION_REDIRECT");
            let mail_user = std::env!("MAIL_USER");
            let mail_pass = std::env!("MAIL_PASS");

            Self {
                database_url,
                stripe_secret,
                stripe_key,
                completion_redirect,
                mail_user,
                mail_pass,
            }
        }
    }
}
