use crate::{api::stripe::UserData, Mailer};
use actix_web::Result;

use lettre::{
    message::{MultiPart, SinglePart},
    AsyncTransport, Message,
};

use super::templates::{Confirmation, Template};
use std::sync::Arc;

pub async fn send_confirmation(user: UserData, mailer: Arc<Mailer>) -> Result<(), String> {
    let confirmation = Confirmation::from(&user);
    let html = SinglePart::html(confirmation.render().unwrap());
    let plaintext = SinglePart::plain(confirmation.render_plaintext());

    let email = Message::builder()
        .from("Kiggyshop <kiggyshop@gmail.com>".parse().unwrap())
        .to("Fay <faycarsons23@gmail.com>".parse().unwrap())
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
