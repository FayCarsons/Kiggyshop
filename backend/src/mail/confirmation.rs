pub use askama::Template;
use model::Quantity;

use crate::api::stripe;
use model::item;

pub struct Item {
    title: String,
    price: f64,
    quantity: u32,
    total: u32,
}

impl From<(&item::Item, &Quantity)> for Item {
    fn from((item, quantity): (&item::Item, &Quantity)) -> Self {
        let price = item.price();
        Self {
            title: item.title.clone(),
            price: price as f64 / 100f64,
            quantity: *quantity,
            total: price * quantity,
        }
    }
}

#[derive(Template)]
#[template(path = "./confirmation.html")]
pub struct Confirmation {
    name: String,
    #[allow(unused)]
    // NOTE: address not currently used, email should echo address back to user
    // so they can verify it
    address: String,
    total: f64,
    cart: Vec<Item>,
}

impl Confirmation {
    pub fn render_plaintext(&self) -> String {
        let Confirmation {
            name, total, cart, ..
        } = self;

        let mut table = prettytable::Table::new();
        table.add_row(prettytable::row![
            "Title".to_string(),
            "Price".to_string(),
            "Quantity".to_string(),
            "Total".to_string(),
        ]);

        for item in cart {
            table.add_row(prettytable::row![
                item.title.clone(),
                item.price.to_string(),
                item.quantity.to_string(),
                item.total.to_string(),
            ]);
        }

        table.add_empty_row();
        table.add_row(prettytable::row!["Total", "", "", total]);

        format!(
            "Thank you {name}!\n\nWe appreciate your support! Your order is currently being processed, a shipping confirmation will be sent shortly.\n\n{table}"
        )
    }
}

impl From<&stripe::User> for Confirmation {
    fn from(
        stripe::User {
            name,
            address,
            total,
            cart,
            ..
        }: &stripe::User,
    ) -> Self {
        let cart = cart
            .iter()
            .map(
                |(
                    _,
                    stripe::Item {
                        title,
                        price,
                        quantity,
                    },
                )| Item {
                    title: title.clone(),
                    price: (*price as f64) / 100f64,
                    quantity: *quantity,
                    total: *total,
                },
            )
            .collect::<Vec<Item>>();

        let address = address
            .clone()
            .map(|addr| addr.to_string())
            .unwrap_or("Address not present".to_string());

        Confirmation {
            name: name.clone(),
            address,
            total: (*total as f64) / 100f64,
            cart,
        }
    }
}
