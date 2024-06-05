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
    address: String,
    total: f64,
    cart: Vec<Item>,
}

impl Confirmation {
    pub fn render_plaintext(&self) -> String {
        let Confirmation {
            name,
            total: order_total,
            cart,
            ..
        } = self;

        let title = "Title";
        let price = "Price";
        let quantity = "Quantity";
        let total = "Total";

        let [title_width, price_width, quantity_width, total_width] = cart.iter().fold(
            [title.len(), price.len(), quantity.len(), total.len()],
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
        .fold(format!("{separator}\n{header}\n"),
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
            String::from("Total: ") + &order_total.to_string(),
            total_width = [title_width, price_width, quantity_width, total_width]
                .into_iter()
                .sum()
        );

        format!(
            "Thank you {name}!\n\nWe appreciate your support! Your order is currently being processed, a shipping confirmation will be sent shortly.\n\n{order_details}\n{total_box}"
        )
    }
}

impl From<&stripe::UserData> for Confirmation {
    fn from(
        stripe::UserData {
            name,
            address,
            total,
            cart,
            ..
        }: &stripe::UserData,
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
