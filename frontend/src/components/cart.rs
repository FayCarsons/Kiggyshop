use common::item::Item;
use web_sys::MouseEvent;
use yew::{
    function_component, html, use_context, Callback, Html, HtmlResult, Properties, Suspense,
};
use yew_router::prelude::use_navigator;

use crate::{
    components::error::Error,
    context::{AppAction, CartAction},
    hooks::use_item,
    utils::{kind_to_price_category, title_to_path},
    Context, Route,
};

#[function_component(CartPage)]
pub fn cart() -> Html {
    let ctx = use_context::<Context>().unwrap();
    let cart_count = ctx.cart.count();
    let cart_items = ctx
        .cart
        .items
        .iter()
        .map(|(item_id, qty)| {
            html! {if ctx.stock.is_some() {<CartItem {item_id} {qty}/>} else {<Suspense><SuspendCartItem {item_id} {qty}/></Suspense>}}
        })
        .collect::<Html>();

    html! {
        <section class="bg-gradient-to-t from-kiggyred to-kiggypink w-full h-full py-0 my-0">
            <div class="container mx-auto">
                <div class="flex justify-center">
                    <div class="w-3/4 bg-white px-10 py-10">
                        <div class="flex justify-between border-b pb-8">
                            <h1 class="font-semibold text-2xl">{"Cart"}</h1>
                            <h2 class="font-semibold text-2xl">{format!("{cart_count} items")}</h2>
                        </div>
                        <div class="flex mt-10">
                            <h3 class="font-semibold text-gray-600 text-xs uppercase w-2/5">{"product"}</h3>
                            <h3 class="font-semibold text-center text-gray-600 text-xs uppercase w-1/5">{"quantity"}</h3>
                            <h3 class="font-semibold text-center text-gray-600 text-xs uppercase w-1/5 text-center">{"price"}</h3>
                            <h3 class="font-semibold text-center text-gray-600 text-xs uppercase w-1/5 text-center">{"total"}</h3>
                        </div>
                            {cart_items}
                            <button>{"checkout"}</button>
                    </div>

                </div>
            </div>
        </section>
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct CartItemProps {
    pub item_id: i32,
    pub qty: u32,
}

#[function_component(CartItem)]
pub fn cart_item(props: &CartItemProps) -> Html {
    let CartItemProps { item_id, qty } = props;

    let ctx = use_context::<Context>().unwrap();
    let item = ctx.get_item(*item_id).unwrap();

    let onclick = {
        let item_id = item_id.clone();
        let ctx = ctx.clone();
        Callback::from(move |_: MouseEvent| {
            ctx.dispatch(AppAction::UpdateCart(CartAction::RemoveItem(item_id)))
        })
    };

    html! {<CartGuts item={item.clone()} {qty} {onclick}/>}
}

#[function_component(SuspendCartItem)]
pub fn suspend_cart_item(CartItemProps { item_id, qty }: &CartItemProps) -> HtmlResult {
    let ctx = use_context::<Context>().unwrap();
    let item = use_item(item_id)?;

    if item.is_err() {
        return Ok(html! {
            <Error/>
        });
    }

    let item = item.unwrap();

    let onclick = {
        let item_id = item_id.clone();
        Callback::from(move |_| {
            ctx.dispatch(AppAction::UpdateCart(CartAction::RemoveItem(item_id)))
        })
    };

    Ok(html! {<CartGuts {item} {qty} {onclick}/>})
}

#[derive(Clone, PartialEq, Properties)]
struct CartGutsProps {
    item: Item,
    qty: u32,
    onclick: Callback<MouseEvent>,
}

#[function_component(CartGuts)]
fn cart_html(CartGutsProps { item, qty, onclick }: &CartGutsProps) -> Html {
    let navigator = use_navigator().unwrap();

    let img_onclick = {
        let id = item.id.clone();
        move |_: MouseEvent| navigator.push(&Route::Product { id })
    };

    let Item { title, kind, .. } = item;

    let (price, _) = kind_to_price_category(kind);

    html! {
    <div class="flex items-center hover:bg-slate-50 -mx-8 px-6 py-5">
        <div class="flex w-full">
            <div class="w-20">
                <img class="h-24 w-24" src={title_to_path(&item.title)} onclick={img_onclick} />
            </div>
            <div class="flex flex-col justify-between ml-4 flex-grow">
                <span class="font-bold text-sm">{title}</span>
                <span class="text-red-500 text-xs">{kind}</span>
                <button class="text-red-500 hover:text-red-700 focus:outline-none" {onclick}>{"x"}</button>
            </div>
            <div class="flex justify-center">
                <button class="text-gray-600 w-3">{"-"}</button>
                <input class="mx-2 my-auto border text-center w-8 h-8" type="text" value={qty.to_string()}/>
                <button class="text-gray-600 w-3">{"+"}</button>
            </div>
            <span class="text-center w-1/5 font-semibold text-sm">{price}</span>
            <span class="text-center w-1/5 font-semibold text-sm">{price * *qty as f32}</span>
        </div>
    </div>
    }
}

/*
<div class="bg-white rounded-lg p-6 flex items-center space-x-4 shadow-md">

            <div>
                <h2 class="text-lg font-semibold">{item.title.clone()}</h2>
                <p class="text-gray-500">{format!("${}", item.price())}</p>
                <p class="text-gray-500">{format!("qty: {qty}")}</p>

            </div>
        </div>
*/
