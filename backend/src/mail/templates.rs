pub use askama::Template;
use model::Quantity;

use crate::api::stripe::{StripeItem, UserData};

pub struct Item {
    title: String,
    price: f32,
    quantity: u32,
    total: u32,
}

impl From<(&model::item::Item, &Quantity)> for Item {
    fn from((item, quantity): (&model::item::Item, &Quantity)) -> Self {
        let price = item.price();
        Self {
            title: item.title.clone(),
            price: price as f32 / 1000.,
            quantity: *quantity,
            total: price * quantity,
        }
    }
}

#[derive(Template)]
#[template(path = "./confirmation.html")]
pub struct Confirmation {
    name: String,
    address: String,
    total: f32,
    cart: Vec<Item>,
}

impl Confirmation {
    pub fn render_plaintext(&self) -> String {
        let Confirmation {
            name,
            address,
            total,
            cart,
        } = self;

        let [title_width, price_width, quantity_width, total_width] = cart.iter().fold(
            [5, 5, 8, 5],
            |mut acc,
             Item {
                 title,
                 price,
                 quantity,
                 total,
             }| {
                for (idx, column) in [
                    title.clone(),
                    price.to_string(),
                    quantity.to_string(),
                    total.to_string(),
                ]
                .iter()
                .enumerate()
                {
                    acc[idx] = acc[idx].max(column.len())
                }
                acc
            },
        );

        let separator = format!(
            "+-{:-<title_width$}-+-{:-<price_width$}-+-{:-<quantity_width$}-+-{:-<total_width$}-+",
            "",
            "",
            "",
            "",
            title_width = title_width,
            price_width = price_width,
            quantity_width = quantity_width,
            total_width = total_width
        );

        let header = format!(
            "| {:^title_width$} | {:^price_width$} | {:^quantity_width$} | {:^total_width$} |",
            "Title",
            "Price",
            "Quantity",
            "Total",
            title_width = title_width,
            price_width = price_width,
            quantity_width = quantity_width,
            total_width = total_width
        );

        let order_details = cart
        .iter()
        .fold(format!("{separator}\n{header}"),
            |mut acc, Item {
                 title,
                 price,
                 quantity,
                 total,
             }| {
                acc.push_str(
                    &format!("{separator}\n| {:<title_width$} | {:<price_width$} | {:>quantity_width$} | {:>total_width$} |\n", 
                              title, price, quantity, total, title_width = title_width, price_width = price_width, quantity_width = quantity_width, total_width = total_width));
                acc
            }
        );

        let total_box = format!(
            "{separator}\n| {:>total_width$} |\n{separator}",
            String::from("Total: ") + &total.to_string(),
            total_width = title_width + price_width + quantity_width + total_width
        );

        format!(
            r#"
        Thank you {name}!

        We appreciate your support! Your order is currently being processed, a 
        shipping confirmation will be sent shortly.

        {order_details}
        {total_box}
    "#
        )
    }
}

impl From<&UserData> for Confirmation {
    fn from(
        UserData {
            name,
            address,
            total,
            cart,
            ..
        }: &UserData,
    ) -> Self {
        let cart = cart
            .iter()
            .map(
                |(
                    _,
                    StripeItem {
                        title,
                        price,
                        quantity,
                    },
                )| Item {
                    title: title.clone(),
                    price: (*price as f32) / 100.,
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
            total: (*total as f32) / 100.,
            cart,
        }
    }
}
