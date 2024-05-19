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
    }: Confirmation,
) -> String {
    let row_len = items.iter().fold(
        [0; 4],
        |mut acc,
         Item {
             title,
             price,
             quantity,
             total,
         }| {
            for (idx, column) in [
                *title,
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
        title_width = row_len[0],
        price_width = row_len[1],
        quantity_width = row_len[2],
        total_width = row_len[3]
    );

    let header = format!(
        "| {:^title_width$} | {:^price_width$} | {:^quantity_width$} | {:^total_width$} |",
        "Total",
        "Price",
        "Quantity",
        "Total",
        title_width = row_len[0],
        price_width = row_len[1],
        quantity_width = row_len[2],
        total_width = row_len[3]
    );

    let order_details = items
        .iter()
        .fold(format!("{separator}\n{header}"),
            |mut acc, Item {
                 title,
                 price,
                 quantity,
                 total,
             }| {
                acc.push_str(&format!("{separator}\n| {:<title_width$} | {:<price_width$} | {:>quantity_width$} | {:>total_width$} |", title, price, quantity, total, title_width = row_len[0], price_width = row_len[1], quantity_width = row_len[2], total_width = row_len[3]));
                acc
            }
        );

    format!(
        r#"
        Thank you {name}!

        We appreciate your support! Your order is currently being processed, a 
        shipping confirmation will be sent shortly.

        {order_details}
    "#
    )
}

#[test]
fn test_text_mail() {
    let cart = vec![
        Item {
            title: String::from("Apple"),
            price: 100,
            quantity: 10,
            total: 1000,
        },
        Item {
            title: String::from("Banana"),
            price: 50,
            quantity: 20,
            total: 1000,
        },
        Item {
            title: String::from("Orange"),
            price: 75,
            quantity: 15,
            total: 1125,
        },
        Item {
            title: String::from("Milk"),
            price: 200,
            quantity: 5,
            total: 1000,
        },
        Item {
            title: String::from("Bread"),
            price: 150,
            quantity: 7,
            total: 1050,
        },
        Item {
            title: String::from("Eggs"),
            price: 250,
            quantity: 4,
            total: 1000,
        },
        Item {
            title: String::from("Cheese"),
            price: 300,
            quantity: 3,
            total: 900,
        },
        Item {
            title: String::from("Butter"),
            price: 400,
            quantity: 2,
            total: 800,
        },
    ];    

    let total = cart.iter().map(|Item {  total, .. }| total).sum();

    let order = Confirmation {
        name: "peeper",
        address: "1400 fourteenhundred st",
        total, 
        cart,
    }
}
