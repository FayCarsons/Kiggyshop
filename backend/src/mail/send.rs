use crate::{api::stripe::UserData, Env};
use actix_web::{HttpResponse, Result};
use model::cart::JsonCart;
use serde::Serialize;

#[derive(Default, Clone, Hash, Debug, Serialize)]
pub struct Email {
    to: String,
    body: String,
}

pub async fn send_confirmation(
    env: actix_web::web::Data<Env<'_>>,
    UserData {
        name,
        address,
        email,
        cart,
    }: UserData,
) -> Result<()> {
    let Env {
        mailgun_user,
        mailgun_pass,
        ..
    } = *env.into_inner();

    let client = awc::Client::default();

    let res = client
        .get("https://api.mailgun.net/v3/kiggyshop/messages")
        .insert_header(("from", "Kristen Rankin <postmaster@kiggyshop.com>"))
        .insert_header(("to", email))
        .insert_header(("subject", "Thank you!"));

    Ok(())
}
