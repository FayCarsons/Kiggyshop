use std::{borrow::Borrow, collections::HashMap, num::ParseIntError};

use actix_web::{
    error, post,
    web::{self, Json},
    HttpRequest, HttpResponse, Result,
};

use stripe::{
    Client, CreatePaymentLink, CreatePaymentLinkAfterCompletion,
    CreatePaymentLinkAfterCompletionRedirect, CreatePaymentLinkLineItems,
    CreatePaymentLinkShippingAddressCollection,
    CreatePaymentLinkShippingAddressCollectionAllowedCountries, CreatePaymentLinkShippingOptions,
    CreatePrice, CreateProduct, CreateShippingRate, CreateShippingRateDeliveryEstimate,
    CreateShippingRateDeliveryEstimateMaximum, CreateShippingRateDeliveryEstimateMaximumUnit,
    CreateShippingRateDeliveryEstimateMinimum, CreateShippingRateFixedAmount, Currency,
    EventObject, EventType, PaymentLink, Price, Product, Shipping, ShippingRate,
    ShippingRateTaxBehavior, ShippingRateType, Webhook,
};

use crate::{
    api::{
        order::insert_order,
        stock::{dec_items, item_from_db},
    },
    utils::print_red,
    DbPool, Env,
};

use model::{item::Item, CartMap, ItemId, Quantity};

pub struct UserData {
    pub name: String,
    pub address: String,
    pub email: String,
    pub cart: HashMap<ItemId, Quantity>,
}

#[post("/checkout")]
pub async fn checkout(
    cart: Json<HashMap<ItemId, Quantity>>,
    pool: web::Data<DbPool>,
    env: web::Data<Env<'_>>,
) -> Result<HttpResponse> {
    let mut item_map = HashMap::<Item, Quantity>::new();
    for (id, qty) in cart.iter() {
        let item = item_from_db(*id, &pool).await?;
        item_map.insert(item, *qty);
    }

    let client = Client::new(env.stripe_secret);

    let mut product_price_pairs = Vec::<(Price, u64)>::with_capacity(item_map.keys().len());

    for (item, qty) in &item_map {
        let mut create_product = CreateProduct::new(&item.title);
        create_product.description = Some(&item.description);
        let product = Product::create(&client, create_product)
            .await
            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(stripe::IdOrCreate::Id(&product.id));
        create_price.unit_amount = Some(item.price() * 100);
        create_price.expand = &["product"];

        let price = Price::create(&client, create_price)
            .await
            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

        product_price_pairs.push((price, u64::from(*qty)));
    }

    let shipping = {
        let rate = CreateShippingRate {
            metadata: None,
            delivery_estimate: Some(CreateShippingRateDeliveryEstimate {
                maximum: Some(CreateShippingRateDeliveryEstimateMaximum {
                    unit: CreateShippingRateDeliveryEstimateMaximumUnit::Week,
                    value: 2,
                }),
                minimum: Some(CreateShippingRateDeliveryEstimateMinimum {
                    unit: stripe::CreateShippingRateDeliveryEstimateMinimumUnit::BusinessDay,
                    value: 5,
                }),
            }),
            display_name: "priority",
            fixed_amount: Some(CreateShippingRateFixedAmount {
                amount: 1000,
                currency: Currency::USD,
                ..Default::default()
            }),
            expand: &[],
            tax_behavior: Some(ShippingRateTaxBehavior::Exclusive),
            tax_code: None,
            type_: Some(ShippingRateType::FixedAmount),
        };
        ShippingRate::create(&client, rate)
    }
    .await
    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let payment_link = {
        let mut create_payment_link = CreatePaymentLink::new(
            product_price_pairs
                .iter()
                .map(|(price, qty)| CreatePaymentLinkLineItems {
                    quantity: *qty,
                    price: price.id.to_string(),
                    adjustable_quantity: None,
                })
                .collect::<Vec<_>>(),
        );
        create_payment_link.metadata = Some(
            cart.iter()
                .map(|(id, qty)| (id.to_string(), qty.to_string()))
                .collect::<HashMap<String, String>>(),
        );

        create_payment_link.shipping_options = Some(vec![CreatePaymentLinkShippingOptions {
            shipping_rate: Some(shipping.id.to_string()),
        }]);

        create_payment_link.after_completion = Some(CreatePaymentLinkAfterCompletion {
            type_: stripe::CreatePaymentLinkAfterCompletionType::Redirect,
            redirect: Some(CreatePaymentLinkAfterCompletionRedirect {
                url: env.completion_redirect.to_string(),
            }),
            hosted_confirmation: None,
        });

        create_payment_link.shipping_address_collection =
            Some(CreatePaymentLinkShippingAddressCollection {
                allowed_countries: vec![
                    CreatePaymentLinkShippingAddressCollectionAllowedCountries::Us,
                ],
            });

        PaymentLink::create(&client, create_payment_link)
    }
    .await
    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body(payment_link.url))
}

/// Receives (all) webhooks
#[post("/stripe_webhooks")]
pub async fn webhook_handler(
    req: HttpRequest,
    payload: web::Bytes,
    pool: web::Data<DbPool>,
    env: web::Data<Env<'_>>,
) -> Result<HttpResponse> {
    println!("INSERTING INTO DATABASE VIA STRIPE WEBHOOKS");

    parse_webhook(req, payload, pool, env.clone())
        .await
        .map(|_| HttpResponse::Ok().finish())
}

/// Determines whether webhook is correct type: is a completed checkout session
pub async fn parse_webhook(
    req: HttpRequest,
    payload: web::Bytes,
    pool: web::Data<DbPool>,
    env: web::Data<Env<'_>>,
) -> Result<()> {
    print_red("", "CURRENTLY IN 'handle_webhook'");

    let payload_str = std::str::from_utf8(payload.borrow())
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let stripe_sig = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_sig, env.stripe_key) {
        if let EventType::CheckoutSessionCompleted = event.type_ {
            if let EventObject::CheckoutSession(session) = event.data.object {
                handle_checkout(session, pool).await?;
            }
        }
    } else {
        print_red(
            "",
            "FAILED TO CONSTRUCT WEBHOOK EVENT, MAYHAPS UR KEY IS WRONG",
        );
    }

    Ok(())
}

/// Takes data from completed checkout session, stores it in DB and updates stock
async fn handle_checkout(session: stripe::CheckoutSession, pool: web::Data<DbPool>) -> Result<()> {
    let shipping_info = session.shipping_details.unwrap();
    let Shipping { address, name, .. } = shipping_info;

    let address = address.ok_or_else(|| {
        error::ErrorInternalServerError(format!(
            "Address not present in Stripe Order for {}",
            name.clone().unwrap_or(String::from("")),
        ))
    })?;

    let name = name.ok_or(error::ErrorInternalServerError("No name for order"))?;

    // Collecting user cart from session metadata
    let cart = session.metadata.unwrap();
    let cart = cart
        .iter()
        .map(|(id, qty)| {
            let id = str::parse::<u32>(id)?;
            let qty = str::parse::<u32>(qty)?;
            Ok((id, qty))
        })
        .collect::<Result<CartMap, ParseIntError>>()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Cannot parse user cart"))?;

    let cart_conn = pool.get().unwrap();
    let stock_conn = pool.get().unwrap();
    let order = insert_order(
        cart_conn,
        cart.clone(),
        name,
        session.customer_email,
        address,
    );

    let stock = dec_items(cart, stock_conn);

    order.await?;
    stock.await?;

    print_red("", "WEBHOOK COMPLETED");

    Ok(())
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}
