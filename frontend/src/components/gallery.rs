use crate::{
    components::{error::Error, footer::Footer, product::ProductPage},
    context::AppAction,
    hooks::use_stock,
    Context, Route,
};

use super::{header::Header, product::GalleryProduct};
use common::item::Item;
use yew::{function_component, html, use_context, Html, HtmlResult};
use yew_router::prelude::use_navigator;

#[function_component(Gallery)]
pub fn gallery() -> HtmlResult {
    let stock = use_stock()?;

    let stock = match stock {
        Ok(s) => s.clone(),
        Err(_) => return Ok(html! {<Error/>}),
    };

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
                <GalleryProduct key={*id} product={item.clone()} onclick={onclick.clone()}/>
            }
        })
        .collect::<Html>();

    let count = ctx.cart.count();

    Ok(html! {
        <div class="bg-slate-50">
            <Header {count}/>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
                    {items}
                </div>
            <Footer/>
        </div>
    })
}
