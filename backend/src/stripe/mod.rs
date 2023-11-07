use std::collections::HashMap;

use actix_web::{
    post,
    web::{Json, Redirect},
};
use common::cart::Cart;
use stripe::{
    Client, CreatePaymentLink, CreatePaymentLinkLineItems, CreatePrice, CreateProduct, Currency,
    PaymentLink, Price, Product, CreatePaymentLinkAfterCompletionRedirect, CreatePaymentLinkAfterCompletion, CreatePaymentLinkShippingAddressCollection, CreatePaymentLinkShippingAddressCollectionAllowedCountries,
};

use crate::{error::ShopResult, ENV};

#[post("/checkout")]
pub async fn checkout(cart: Json<Cart>) -> ShopResult<Redirect> {
    let env = ENV.get().cloned().unwrap_or_default();

    let secret_key = &env.stripe_secret_key;
    let client = Client::new(secret_key);

    let mut metadata = HashMap::from([(String::from("async-stripe"), String::from("true"))]);
    metadata.shrink_to_fit();

    let mut product_price_pairs = Vec::<(Price, u64)>::with_capacity(cart.inner.keys().len());

    for (item, qty) in cart.inner.iter() {
        let mut create_product = CreateProduct::new(&item.title);
        create_product.metadata = Some(metadata.clone());
        let product = Product::create(&client, create_product).await?;

        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(stripe::IdOrCreate::Id(&product.id));
        create_price.metadata = Some(metadata.clone());
        create_price.unit_amount = Some(item.price());
        create_price.expand = &["product"];

        let price = Price::create(&client, create_price).await?;

        product_price_pairs.push((price, *qty));
    }

    let payment_link = {
        let mut create_payment_link = CreatePaymentLink::new(
            product_price_pairs
                .iter()
                .map(|(price, qty)| CreatePaymentLinkLineItems {
                    quantity: *qty,
                    price: price.id.to_string(),
                    ..Default::default()
                })
                .collect::<Vec<_>>());

        create_payment_link.after_completion = Some(CreatePaymentLinkAfterCompletion {
            type_: stripe::CreatePaymentLinkAfterCompletionType::Redirect,
            redirect: Some(CreatePaymentLinkAfterCompletionRedirect {
                url: env.completion_redirect
            }),
            ..Default::default()
        });

        create_payment_link.shipping_address_collection = Some(CreatePaymentLinkShippingAddressCollection {
            allowed_countries: vec![CreatePaymentLinkShippingAddressCollectionAllowedCountries::Us]
        });

        PaymentLink::create(&client, create_payment_link)
    }
    .await?;

    Ok(Redirect::to(payment_link.url))
}
