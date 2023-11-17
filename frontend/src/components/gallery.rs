use crate::{
    components::{dropdown::CartDropdown, error::Error, footer::Footer},
    context::AppAction,
    hooks::use_stock,
    Context, Route,
};

use super::{header::Header, product_card::ProductCard};
use common::item::Item;
use gloo::console::log;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, use_state, Callback, Html, HtmlResult};
use yew_router::prelude::use_navigator;

#[function_component(Gallery)]
pub fn gallery() -> HtmlResult {
    log!("gallery is rendering");
    let stock = use_stock()?;

    let mut stock = match stock {
        Ok(s) => s.clone(),
        Err(_) => return Ok(html! {<Error/>}),
    };
    stock.shrink_to_fit();

    let ctx = use_context::<Context>().unwrap();
    if ctx.stock.is_none() {
        let stock = stock.clone();
        ctx.dispatch(AppAction::LoadStock(stock))
    };

    let navigator = use_navigator().unwrap();
    let onclick = { move |item: Item| navigator.push(&Route::Product { id: item.id }) };

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

    Ok(html! {
        <div class="relative flex bg-slate-50">
                <div class="bg-kiggygreen hidden md:flex md:flex-col items-start top-0 left-0">
                    <CartDropdown onclick={None::<Callback<MouseEvent>>}/>
                </div>
                <div label="main-content" class="flex-1 max-w-full">
                    <Header show_cart={*show_cart} onclick={set_cart.clone()}/>
                    <div label="product-gallery" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
                        {items}
                    </div>
                    if ! *show_cart {<Footer/>}
                </div>

                if *show_cart {
                    <div class="bg-kiggygreen flex flex-col items-end top-0 right-0 md:hidden">
                        <CartDropdown onclick={set_cart}/>
                    </div>
                }
        </div>
    })
}
