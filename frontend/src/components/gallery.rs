use crate::{
    components::{error::Error, footer::Footer, cart::CartDropdown},
    context::AppAction,
    hooks::use_stock,
    Context, Route,
};

use super::{header::Header, product_card::ProductCard};
use common::item::Item;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, Html, HtmlResult, Callback, use_state};
use yew_router::prelude::use_navigator;

#[function_component(Gallery)]
pub fn gallery() -> HtmlResult {
    let stock = use_stock()?;

    let mut stock = match stock {
        Ok(s) => s.clone(),
        Err(_) => return Ok(html! {<Error/>}),
    };

    stock.shrink_to_fit();

    let ctx = use_context::<Context>().unwrap();
    {
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
        move |_:MouseEvent| {
            show_cart.set(! *show_cart);
        }
    };

    Ok(html! {
        <div class="flex">
                <div class="fixed left-0 ml-auto w-64 h-full hidden md:block z-0 transition-all duration-300 ease-in-out">
                    <CartDropdown onclick={None::<Callback<MouseEvent>>}/>
                </div>
                <div class="container flex-grow max-w-full bg-slate-50 z-0">
                    <Header onclick={set_cart.clone()}/>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
                        {items}
                    </div>
                    <Footer/>
                </div>
                if *show_cart {
                    <div class={format!("fixed right-0 w-64 h-full z-0 transition-all duration-300 ease-in-out {}", if *show_cart {"block"} else {"hidden"})}>
                        <CartDropdown onclick={set_cart}/>
                    </div>
                }
        </div>
    })
}
