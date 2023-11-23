use crate::{
    components::{dropdown::{CartDropdown, LEFT_DROPDOWN_CLASS, RIGHT_DROPDOWN_CLASS}, error::Error, footer::Footer},
    context::AppAction,
    hooks::use_stock,
    Context, Route,
};

use super::{header::Header, product_card::ProductCard};
use common::ItemId;
use gloo::console::log;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, use_state, Callback, Html, HtmlResult};
use yew_router::prelude::use_navigator;

#[function_component(Gallery)]
pub fn gallery() -> HtmlResult {
    log!("gallery is rendering");
    let stock = use_stock()?;

    let stock = match stock {
        Ok(s) => s.clone(),
        Err(_) => return Ok(html! {<Error/>}),
    };

    let ctx = use_context::<Context>().unwrap();
    if ctx.stock.is_none() {
        let stock = stock.clone();
        ctx.dispatch(AppAction::LoadStock(stock))
    };

    let navigator = use_navigator().unwrap();
    let onclick = { move |id: ItemId| navigator.push(&Route::Product { id }) };

    let items = stock
        .iter()
        .map(|(id, item)| {
            html! {
                <ProductCard key={*id} product={item.clone()} onclick={onclick.clone()}/>
            }
        })
        .collect::<Html>();

    //let count = ctx.cart.count();

    let show_cart = use_state(|| false);
    let set_cart = {
        let show_cart = show_cart.clone();
        move |_: MouseEvent| {
            show_cart.set(!*show_cart);
        }
    };

    let _left_dropdown_class = "min-h-screen top-0 p-4 w-0 md:w-52 transition-all duration-300 ease-in-out bg-kiggygreen hidden md:flex flex-col items-start top-0 left-0";

    Ok(html! {
        <div class="relative flex bg-slate-50">
            <CartDropdown onclick={None::<Callback<MouseEvent>>} class={LEFT_DROPDOWN_CLASS}/>
            <div label="main-content" class="flex-1 max-w-full">
                <Header show_cart={*show_cart} onclick={set_cart.clone()}/>
                <div label="product-gallery" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
                    {items}
                </div>
                if ! *show_cart {<Footer/>}
            </div>

            if *show_cart {
                <CartDropdown onclick={set_cart} class={RIGHT_DROPDOWN_CLASS}/>
            }
        </div>
    })
}
