use std::collections::HashMap;

use actix_web::{
    post,
    web::{Json, Redirect},
};
use common::cart::Cart;
use stripe::{
    Client, CreatePaymentLink, CreatePaymentLinkLineItems, CreatePrice, CreateProduct, Currency,
    PaymentLink, Price, Product,
};

use crate::{error::ShopResult, STRIPE_SECRET_KEY};

#[post("/checkout")]
pub async fn checkout(cart: Json<Cart>) -> ShopResult<Redirect> {
    let secret_key = STRIPE_SECRET_KEY.get().unwrap();
    let client = Client::new(secret_key);

    let mut metadata = HashMap::from([(String::from("async-stripe"), String::from("true"))]);
    metadata.shrink_to_fit();

    let mut product_price_pairs = Vec::<(Price, u64)>::with_capacity(cart.inner.keys().len());

    for (item, qty) in cart.inner.iter() {
        let mut create_product = CreateProduct::new(&item.title);
        create_product.metadata = Some(metadata.clone());
        let product = Product::create(&client, create_product).await.unwrap();

        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(stripe::IdOrCreate::Id(&product.id));
        create_price.metadata = Some(metadata.clone());
        create_price.unit_amount = Some(item.price());
        create_price.expand = &["product"];

        let price = Price::create(&client, create_price).await.unwrap();

        product_price_pairs.push((price, *qty));
    }

    let payment_link = PaymentLink::create(
        &client,
        CreatePaymentLink::new(
            product_price_pairs
                .iter()
                .map(|(price, qty)| CreatePaymentLinkLineItems {
                    quantity: *qty,
                    price: price.id.to_string(),
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        ),
    )
    .await
    .unwrap();

    Ok(Redirect::to(payment_link.url))
}
