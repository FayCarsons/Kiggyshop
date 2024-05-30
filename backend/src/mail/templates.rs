use askama::Template;

pub struct Item {
    title: String,
    price: u32,
    quantity: u32,
    total: u32,
}

#[derive(Template)]
#[template(path = "./confirmation.html")]
pub struct Confirmation {
    name: String,
    address: String,
    total: u32,
    cart: Vec<Item>,
}

fn text_confirmation(
    Confirmation {
        name, total, cart, ..
    }: &Confirmation,
) -> String {
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
