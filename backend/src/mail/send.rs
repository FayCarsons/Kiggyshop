use crate::{api::stripe::UserData, Mailer};
use actix_web::Result;

use askama::Template;

use lettre::{
    message::{MultiPart, SinglePart},
    AsyncTransport, Message,
};

use super::{confirmation, shipped};

use std::sync::Arc;

pub async fn send_confirmation(user: UserData, mailer: Arc<Mailer>) -> Result<(), String> {
    let confirmation = confirmation::Confirmation::from(&user);
    let html = SinglePart::html(confirmation.render().unwrap());
    let plaintext = SinglePart::plain(confirmation.render_plaintext());

    let email = Message::builder()
        .from("Kiggyshop <kiggyshop@gmail.com>".parse().unwrap())
        .to(user.email.parse().unwrap())
        .subject("Thank you for your order!")
        .multipart(
            MultiPart::alternative()
                .singlepart(html)
                .singlepart(plaintext),
        )
        .unwrap();

    match mailer.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn send_tracking(shipping: shipped::Shipped, mailer: Arc<Mailer>) -> Result<(), String> {
    let html = SinglePart::html(shipping.render().unwrap());
    let plaintext = SinglePart::plain(shipping.render_plaintext());

    let email = Message::builder()
        .from("KiggyShop <kiggyshop@gmail.com>".parse().unwrap())
        .to(shipping.email.parse().unwrap())
        .subject("Your orderhas shipped!")
        .multipart(
            MultiPart::alternative()
                .singlepart(html)
                .singlepart(plaintext),
        )
        .unwrap();

    match mailer.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
