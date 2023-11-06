use crate::{cart::AppAction, components::product::ProductPage, hooks::use_stock, Context, Route};

use super::product::GalleryProduct;
use common::{item::Item, log_debug};
use yew::{
    function_component, html, html::RenderError, suspense::Suspension, use_context, use_state,
    Callback, Html, HtmlResult, UseStateHandle,
};
use yew_router::prelude::Link;

#[function_component(Home)]
pub fn home() -> HtmlResult {
    log_debug!("I am in the Home Component");
    let stock = use_stock()?;

    if stock.is_err() {
        return Ok(html! {
            <div>
                <p>{"there's bean an error :/"}</p>
                <p>{stock.err().unwrap().to_string()}</p>
            </div>
        });
    }

    let stock = stock.unwrap().clone();

    let ctx = use_context::<Context>().unwrap();
    {
        let stock = stock.clone();
        ctx.dispatch(AppAction::LoadStock(stock))
    };

    let cart_count = format!("cart: {}", ctx.cart.count());
    let focus: UseStateHandle<Option<Item>> = use_state(|| None);

    let onclick = || {
        let focus = focus.clone();
        move |item| {
            if focus.is_none() {
                focus.set(Some(item))
            }
        }
    };

    let home = {
        let focus = focus.clone();
        move |_| {
            if focus.is_some() {
                focus.set(None)
            }
        }
    };

    let items = if focus.clone().is_none() {
        stock
            .iter()
            .map(|(_, item)| {
                html! {
                        <GalleryProduct product={item.clone()} onclick={onclick()}/>
                }
            })
            .collect::<Html>()
    } else {
        let product = focus.as_ref().unwrap();
        html! {
                <ProductPage product={product.clone()}/>
        }
    };

    Ok(html! {
        <div>
            <header onclick={home}>
                <h1>{"Kristen Rankin"} </h1>
                <div class="shop-btn" id="cart-btn">
                    <Link<Route> to={Route::Cart}>{cart_count}</Link<Route>>
                </div>
            </header>
            <hr class="separator" />
            <div class={if focus.is_none() {"products"} else {"product-details-container"}}>
                {items}
            </div>
            <footer>
                <button class="shop-btn" id="contact-button">{"Contact me"}</button>
            </footer>
        </div>
    })
}
