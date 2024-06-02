use std::sync::Arc;

use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use model::order::Order;

use crate::{
    api::{
        stock::item_from_db,
        stripe::{StripeItem, UserData},
    },
    mail::{
        self,
        templates::{Confirmation, Item},
    },
    Mailer,
};

#[actix_web::test]
async fn test_confirmation_email() {
    let order = serde_json::from_slice::<Order>(include_bytes!("mock_order.json")).unwrap();

    let items =
        serde_json::from_slice::<Vec<model::item::Item>>(include_bytes!("../../stock.json"))
            .unwrap();

    let Order {
        name,
        email,
        total,
        cart,
        address,
        ..
    } = order;

    let cart = cart
        .iter()
        .map(|(id, qty)| {
            let price = items[*id as usize].price();

            (
                *id,
                StripeItem {
                    price,
                    title: items[*id as usize].title.clone(),
                    quantity: *qty,
                },
            )
        })
        .collect();

    let user = UserData {
        name,
        address: Some(address),
        email,
        total,
        cart,
    };

    let creds = Credentials::new(
        dotenvy_macro::dotenv!("MAIL_USER").to_string(),
        dotenvy_macro::dotenv!("MAIL_PASS").to_string(),
    );
    let mailer: Mailer = Mailer::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    if let Err(e) = mail::send::send_confirmation(user, Arc::new(mailer)).await {
        panic!("Cannot send confirmation test email: {e}");
    }
}
