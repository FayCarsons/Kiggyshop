use std::sync::Arc;

use lettre::transport::smtp::authentication::Credentials;
use model::{item, order};

use crate::{
    api::stripe,
    mail::{self, shipped},
    Mailer,
};

#[actix_web::test]
async fn test_confirmation_email() {
    let order = serde_json::from_slice::<order::Order>(include_bytes!("mock_order.json")).unwrap();

    let items =
        serde_json::from_slice::<Vec<item::Item>>(include_bytes!("../../stock.json")).unwrap();

    let order::Order {
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
                stripe::Item {
                    price,
                    title: items[*id as usize].title.clone(),
                    quantity: *qty,
                },
            )
        })
        .collect();

    let user = stripe::UserData {
        name,
        address: Some(address),
        email,
        total,
        subtotal: total,
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

#[actix_web::test]
async fn test_shipping() {
    let order = serde_json::from_slice::<order::Order>(include_bytes!("mock_order.json"))
        .expect("Cannot parse mock orders");

    let order::Order {
        id,
        name,
        email,
        total,
        ..
    } = order;

    let table_order = order::TableOrder {
        id: id as i32,
        name,
        email,
        total: total as i32,
        shipped: false,
        tracking_number: Some("URSILLY8901".to_string()),
    };
    let shipped =
        shipped::Shipped::try_from(table_order).expect("Cannot convert <TableOrder> to <Shipped>");

    let creds = Credentials::new(
        dotenvy_macro::dotenv!("MAIL_USER").to_string(),
        dotenvy_macro::dotenv!("MAIL_PASS").to_string(),
    );
    let mailer: Mailer = Mailer::relay("smtp.gmail.com")
        .expect("Cannot build mailer")
        .credentials(creds)
        .build();

    if let Err(e) = mail::send::send_tracking(shipped, Arc::new(mailer)).await {
        panic!("Cannot send confirmation test email: {e}");
    }
}
