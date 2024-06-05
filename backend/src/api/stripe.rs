use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, sync::Arc};

use actix_web::{
    error, post, rt,
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
    api::{order::insert_order, stock::dec_items},
    mail,
    utils::print_red,
    DbPool, Env, Mailer,
};

use model::{address::Address, ItemId, Quantity};

use super::stock::get_matching_ids;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub title: String,
    pub price: u32,
    pub quantity: Quantity,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserData {
    pub name: String,
    pub address: Option<Address>,
    pub email: String,
    pub total: u32,
    pub subtotal: u32,
    pub cart: HashMap<ItemId, Item>,
}

#[post("/checkout")]
pub async fn checkout(
    cart: Json<HashMap<ItemId, Quantity>>,
    pool: web::Data<DbPool>,
    env: web::Data<Env<'_>>,
) -> Result<HttpResponse> {
    let ids = cart.keys().copied().collect::<Vec<ItemId>>();
    let items = get_matching_ids(ids, pool.into_inner()).await?;
    let item_map = items
        .into_iter()
        .map(|item| {
            (
                item.id as u32,
                Item {
                    title: item.title.clone(),
                    price: item.price(),
                    quantity: *cart.get(&(item.id as u32)).unwrap(),
                },
            )
        })
        .collect::<HashMap<ItemId, Item>>();

    let client = Client::new(env.stripe_secret);

    let mut product_price_pairs = Vec::<(Price, u64)>::with_capacity(item_map.keys().len());

    for Item {
        title,
        price,
        quantity,
    } in item_map.values()
    {
        let create_product = CreateProduct::new(title);
        let product = Product::create(&client, create_product)
            .await
            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(stripe::IdOrCreate::Id(&product.id));
        create_price.unit_amount = Some(*price as i64);
        create_price.expand = &["product"];

        let price = Price::create(&client, create_price)
            .await
            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

        product_price_pairs.push((price, u64::from(*quantity)));
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
            item_map
                .iter()
                .map(|(id, item)| Ok((id.to_string(), serde_json::to_string(item)?)))
                .collect::<Result<HashMap<String, String>, serde_json::error::Error>>()?,
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

        create_payment_link.customer_creation = Some(stripe::PaymentLinkCustomerCreation::Always);

        PaymentLink::create(&client, create_payment_link)
    }
    .await
    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body(payment_link.url))
}

/// Receives (all) webhooks
#[post("/stripe_webhooks")]
pub async fn webhook(
    req: HttpRequest,
    payload: web::Bytes,
    pool: web::Data<DbPool>,
    env: web::Data<Env<'_>>,
    mailer: web::Data<Mailer>,
) -> Result<HttpResponse> {
    parse_webhook(req, payload, pool.into_inner(), env.into_inner(), mailer)
        .await
        .map(|_| HttpResponse::Ok().finish())
}

/// Determines whether webhook is correct type: is a completed checkout session
pub async fn parse_webhook(
    req: HttpRequest,
    payload: web::Bytes,
    pool: Arc<DbPool>,
    env: Arc<Env<'_>>,
    mailer: web::Data<Mailer>,
) -> Result<()> {
    print_red("", "CURRENTLY IN 'handle_webhook'");

    let payload_str = std::str::from_utf8(payload.borrow()).map_err(|e| {
        error::ErrorInternalServerError(format!("Stripe payload is not UTF-8: {e}"))
    })?;

    let stripe_sig = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    let event = Webhook::construct_event(payload_str, stripe_sig, env.stripe_key);

    if let Ok(event) = event {
        if let EventType::CheckoutSessionCompleted = event.type_ {
            if let EventObject::CheckoutSession(session) = event.data.object {
                handle_checkout(session, pool, mailer.into_inner()).await?;
            }
        }
    } else {
        print_red(
            "FAILED TO CONSTRUCT WEBHOOK EVENT, IS YOUR KEY CORRECT?\nERROR_MESSSAGE:",
            unsafe { &event.unwrap_err_unchecked() },
        );
    }

    Ok(())
}

fn make_address(stripe_address: stripe::Address, name: Arc<str>) -> Result<Address, String> {
    let (number, street) = {
        let full = stripe_address
            .line1
            .map(|line| line + &stripe_address.line2.unwrap_or_default())
            .ok_or("No address field in order")?;
        full.trim()
            .split_once(" ")
            .ok_or(format!("Malformed address: {full}"))
            .and_then(|(number, name)| match str::parse::<u32>(number) {
                Ok(num) => Ok((num, name.to_string())),
                Err(e) => Err(format!("Invalid house number: {e}")),
            })?
    };

    let zipcode = stripe_address
        .postal_code
        .ok_or("No zipcode specified foar order")
        .and_then(|str| str::parse::<u32>(&str).map_err(|_| "Cannot parse zipcode"))?;

    let address = Address {
        number,
        street,
        city: stripe_address.city.unwrap_or("Not specified".to_string()),
        state: stripe_address
            .state
            .unwrap_or("No state specified for order".to_string()),
        zipcode,
        name: name.to_string(),
    };

    Ok(address)
}

/// Takes data from completed checkout session, stores it in DB and updates stock
// TODO: Add more advanced error handling, returning HTTP error response only if
// critical failure occurs, otherwise filling unavailable fields with
// "Not specified"
async fn handle_checkout(
    session: stripe::CheckoutSession,
    pool: Arc<DbPool>,
    mailer: Arc<Mailer>,
) -> Result<()> {
    let shipping_info = session.shipping_details.unwrap();
    let Shipping { address, name, .. } = shipping_info;

    // Collect user info - wrap everything in arc bc its all being passed to multiple async fns
    let stripe_address = address.ok_or_else(|| {
        error::ErrorInternalServerError(format!(
            "Address not present in Stripe Order for {}",
            name.clone().unwrap_or(String::from("")),
        ))
    })?;

    let name = Arc::<str>::from(
        name.unwrap_or("Name not present in Stripe payload".to_string())
            .as_str(),
    );

    let email = Arc::<str>::from(
        session
            .customer_email
            .unwrap_or("Not present".to_string())
            .as_str(),
    );

    // Collecting user cart from session metadata
    let cart = session.metadata.ok_or(error::ErrorBadRequest(
        "Session metadata not present: need user cart",
    ))?;

    let cart = Arc::new(
        cart.iter()
            .map(|(id, item)| {
                let id = str::parse::<u32>(id).map_err(|e| {
                    error::ErrorInternalServerError(format!("Error parsing item id: {e}"))
                })?;
                let item = serde_json::from_str(item)?;
                Ok((id, item))
            })
            .collect::<Result<HashMap<ItemId, Item>, actix_web::Error>>()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Cannot parse user cart"))?,
    );
    let total = session
        .amount_total
        .map(|n| n as u32)
        .unwrap_or_else(|| cart.clone().iter().map(|(_, item)| item.price).sum());
    let subtotal = session.amount_subtotal.unwrap_or_default() as u32;

    #[cfg(debug_assertions)]
    println!("Webhook endpoint received cart: {:#?}", cart);

    let address = Arc::new(make_address(stripe_address, name.clone()).unwrap_or_default());

    // Get DB connections
    let cart_conn = pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(format!("Cannot get DB connection: {e}")))?;
    let stock_conn = pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(format!("Cannot get DB connection: {e}")))?;

    let user_data = UserData {
        name: name.clone().to_string(),
        address: Some((*address).clone()),
        email: email.to_string(),
        total,
        subtotal,
        cart: (*cart).clone(),
    };

    let order = rt::spawn(insert_order(
        cart_conn,
        cart.clone(),
        total,
        name,
        email,
        address,
    ));
    let stock = rt::spawn(dec_items(cart, stock_conn));
    let confirmation = rt::spawn(mail::send::send_confirmation(user_data, mailer));

    // TODO: Improve w exponential backoff
    match (order.await, stock.await, confirmation.await) {
        (Err(e), _, _) => Err(error::ErrorInternalServerError(format!(
            "Saving order failed: {e}"
        ))),
        (_, Err(e), _) => Err(error::ErrorInternalServerError(format!(
            "Updating stock failed: {e}"
        ))),
        (_, _, Err(e)) => Err(error::ErrorInternalServerError(format!(
            "Sending confirmation email failed: {e}"
        ))),
        _ => Ok(()),
    }
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}
