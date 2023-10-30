use crate::{components::product::FocusProduct, hooks::use_stock};

use super::product::GalleryProduct;
use common::item::Item;
use gloo::console::log;
use yew::{function_component, html, use_state, Callback, Html, HtmlResult};

#[function_component(Home)]
pub fn home() -> HtmlResult {
    log!("I am in the Home Component");
    let stock = use_stock()?;

    if stock.is_err() {
        return Ok(html! {
            <div>
                <p>{"there's bean an error :/"}</p>
                <p>{stock.err().unwrap().to_string()}</p>
            </div>
        })
    }

    let focus: yew::UseStateHandle<Option<Item>> = use_state(|| None);

    let onclick = || {
        let focus = focus.clone();
        Callback::from(move |item| {
            if focus.is_none() {
                focus.set(Some(item))
            }
        })
    };

    let home = {
        let focus = focus.clone();
        Callback::from(move |_| {
            if focus.is_some() {
                focus.set(None)
            }
        })
    };

    let items = if focus.clone().is_none() {
        stock
            .unwrap()
            .iter()
            .map(|item| {
                html! {
                        <GalleryProduct product={item.clone()} onclick={onclick()}/>
                }
            })
            .collect::<Html>()
    } else {
        let product = focus.as_ref().unwrap();
        html! {
                <FocusProduct product={product.clone()}/>
        }
    };

    Ok(html! {
        <div>
            <header onclick={home}>
                <h1>{"Kristen Rankin"} </h1>
            </header>
            <hr class="separator" />
            <div class={if focus.is_none() {"products"} else {"product-details-container"}}>
                {items}
            </div>
            <footer>
                <button class="contact-button">{"Contact me"}</button>
            </footer>
        </div>
    })
}
