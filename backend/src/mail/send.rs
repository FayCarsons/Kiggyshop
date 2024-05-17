use crate::Env;
use model::cart::JsonCart;
use actix_web::{HttpResponse, Result};
use serde::Serialize;

#[derive(Default, Clone, Hash, Debug, Serialize)]
pub struct Email {
    to: String,
    body: String,
}

pub async fn send_confirmation(
    env: actix_web::web::Data<Env>,
    user_data: UserData,
    cart: JsonCart
) -> Result<HttpResponse> {
    let Env { mailgun_user, mailgun_pass, .. } = *env.into_inner();

    let client = awc::Client::default();

    let res = client
        .get("https://api.mailgun.net/v3/kiggyshop/messages")
        .insert_header(("from", "Kristen Rankin <postmaster@kiggyshop.com>"))
        .insert_header(("to", ))
    }
