use crate::{
    components::{cart::CartDropdown, error::Error, footer::Footer},
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
        <div class="flex w-full h-full">
                <div class="fixed flex-none left-0 ml-0 min-w-64 h-full hidden md:block">
                    <CartDropdown onclick={None::<Callback<MouseEvent>>}/>
                </div>

                if *show_cart {
                    <div class="fixed flex-none block min-w-64 h-full">
                        <CartDropdown onclick={set_cart.clone()}/>
                    </div>
                }

                <div class="flex-auto max-w-full bg-slate-50">
                    <Header onclick={set_cart}/>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
                        {items}
                    </div>
                    <Footer/>
                </div>
        </div>
    })
}
