#[derive(Clone, Debug, Default)]
pub struct Env<'a> {
    pub database_url: &'a str,
    pub stripe_secret: &'a str,
    pub stripe_key: &'a str,
    pub completion_redirect: &'a str,
    pub mailgun_user: &'a str,
    pub mailgun_pass: &'a str,
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
            let mailgun_user = dotenvy_macro::dotenv!("MAILGUN_USER");
            let mailgun_pass = dotenvy_macro::dotenv!("MAILGUN_PASS");
            Ok(Self {
                database_url,
                stripe_secret,
                stripe_key,
                completion_redirect,
                mailgun_user,
                mailgun_pass,
            })
        }
        #[cfg(not(debug_assertions))]
        {
            // Building in Docker container
            let database_url = std::env!("REMOTE_DATABASE_PATH");
            let stripe_secret = std::env!("STRIPE_SECRET");
            let stripe_key = std::env!("STRIPE_KEY");
            let completion_redirect = std::env!("COMPLETION_REDIRECT");
            let mailgun_user = std::env!("MAILGUN_USER");
            let mailgun_pass = std::env!("MAILGUN_PASS");
            Ok(Self {
                database_url,
                stripe_secret,
                stripe_key,
                completion_redirect,
                mailgun_user,
                mailgun_pass,
            })
        }
    }
}
