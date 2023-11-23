use std::rc::Rc;

use common::item::FrontEndItem;

use web_sys::MouseEvent;
use yew::{function_component, html, use_context, Callback, Html};
use yew_router::prelude::use_navigator;

use crate::{
    components::error::Error,
    context::{AppAction, CartAction},
    utils::{checkout, get_quantity_element, kind_to_price, title_to_path, SHIPPING_COST},
    Context, Route,
};

use super::dropdown::CartItemProps;

#[function_component(CartPage)]
pub fn cart_page() -> Html {
    let ctx = use_context::<Context>().unwrap();

    let products = ctx
        .cart
        .items
        .iter()
        .map(|(id, qty)| {
            if *qty == 0 {
                ctx.dispatch(AppAction::UpdateCart(Rc::new(CartAction::RemoveItem(*id))));
                return html! {<></>};
            }
            let item = ctx.get_item(id);

            match item {
                Some(item) => {
                    let (item, quantity) = (item.clone(), *qty);
                    html! {<CartPageItem {item} {quantity}/>}
                }
                None => html! {<Error/>},
            }
        })
        .collect::<Html>();

    let total = ctx.get_total() + SHIPPING_COST;

    let checkout = checkout(ctx.cart.clone());

    html! {
        <div class="absolute min-h-screen top-0 right-0 left-0 min-w-screen bg-gradient-to-b from-kiggypink to-kiggyred">

            <div class="container w-3/4 mx-auto my-8  bg-slate-50">
                <div class="p-8 rounded-md shadow-md">
                    <h2 class="text-2xl font-semibold mb-4">{"cart"}</h2>

                    /* Cart Items List */
                    <div class="space-y-4">
                    {products}
                    </div>

                    /* Total and Checkout Section */
                    <div class="flex justify-between items-center mt-8">
                        <div>
                            <p class="text-xl font-semibold">{format!("Total: ${total}")}</p>
                            <p class="text-gray-500 text-sm">{"shipping: $10"}</p>
                        </div>
                        <button class="bg-kiggygreen text-white px-6 py-2 rounded-md hover:brightness-90" onclick={checkout}>
                            {"checkout"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(CartPageItem)]
pub fn cart_page_item(CartItemProps { item, quantity }: &CartItemProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let FrontEndItem {
        id,
        title,
        kind,
        stock,
        ..
    } = item;

    let price = kind_to_price(kind);

    let make_cart_callback = {
        move |action: CartAction| {
            let ctx = ctx.clone();
            let action = Rc::new(action);
            move |_: MouseEvent| ctx.dispatch(AppAction::UpdateCart(action.clone()))
        }
    };

    let navigator = use_navigator().unwrap();

    let product_callback = {
        let id = *id;
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.push(&Route::Product { id }))
    };

    html! {
        <div class="flex items-center justify-between border-b border-kiggygreen py-4">
            /* Left Section: Image, Title, Price */
            <div class="flex items-center space-x-4">
              <img
                class="w-16 h-16 object-cover rounded-md"
                src={title_to_path(title)}
                alt="Product"
                onclick={product_callback.clone()}
              />
              <div>
                <p class="text-gray-800 text-lg font-semibold" onclick={product_callback}>{title}</p>
                <p class="text-gray-500">{format!("${price}")}</p>
                /* Only x left in stock message */
                {get_quantity_element(stock)}
              </div>
            </div>

            /* Right Section: Quantity Controls and Remove Button */
            <div class="flex items-center space-x-4">
              /* Quantity Controls */
              <div class="flex items-center space-x-2">
                <button class="text-gray-600 focus:outline-none" onclick={make_cart_callback(CartAction::DecItem(*id))}>
                  <span class="text-xl">{"-"}</span>
                </button>
                <span class="text-gray-800">{quantity}</span>
                <button class="text-gray-600 focus:outline-none" onclick={make_cart_callback(CartAction::IncItem(*id))}>
                  <span class="text-xl">{"+"}</span>
                </button>
              </div>

              /* Remove Button */
              <button class="text-red-500 focus:outline-none" onclick={make_cart_callback(CartAction::RemoveItem(*id))}>
                <span class="text-md">{"x"}</span>
              </button>
            </div>
          </div>
    }
}
