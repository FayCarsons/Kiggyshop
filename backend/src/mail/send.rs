use crate::{api::stripe::UserData, Env};
use actix_web::Result;
use serde::Serialize;

use super::templates::{Confirmation, Template};

#[derive(Default, Clone, Hash, Debug, Serialize)]
pub struct Email {
    to: String,
    body: String,
}

pub async fn send_confirmation(env: &Env<'_>, user: UserData) -> Result<()> {
    let client = awc::Client::default();

    let Env {
        mailgun_user,
        mailgun_pass,
        ..
    } = env;

    let confirmation = Confirmation::from(&user);
    let html = confirmation.render();
    let plaintext = confirmation.render_plaintext();

    let res = client
        .get("https://api.mailgun.net/v3/kiggyshop/messages")
        .basic_auth(mailgun_user, mailgun_pass)
        .insert_header(("from", "Kristen Rankin <postmaster@kiggyshop.com>"))
        .insert_header(("to", user.email))
        .insert_header(("subject", "Thank you for your order!"));

    Ok(())
}
