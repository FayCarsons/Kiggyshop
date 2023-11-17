use std::rc::Rc;

use common::item::Item;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, Callback, Html};
use yew_router::prelude::Link;

use crate::{
    components::{error::Error, header::Header},
    context::{AppAction, CartAction},
    utils::{kind_to_price, title_to_path},
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

    let total = ctx.get_total() + 10.;

    let show_cart = false;
    let onclick = |_: MouseEvent| {};

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
                        <button class="bg-kiggygreen text-white px-6 py-2 rounded-md hover:brightness-90">
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

    let Item {
        id,
        title,
        kind,
        quantity: in_stock,
        ..
    } = item;

    let price = kind_to_price(kind);

    let make_cart_callback = {
        move |action: CartAction| {
            let ctx = ctx.clone();
            let action = Rc::new(action);
            Callback::from(move |_: MouseEvent| ctx.dispatch(AppAction::UpdateCart(action.clone())))
        }
    };

    html! {
        <div class="flex items-center justify-between border-b border-kiggygreen py-4">
            /* Left Section: Image, Title, Price */
            <div class="flex items-center space-x-4">
              <img
                class="w-16 h-16 object-cover rounded-md"
                src={title_to_path(title)}
                alt="Product"
              />
              <div>
                <p class="text-gray-800 text-lg font-semibold">{title}</p>
                <p class="text-gray-500">{format!("${price}")}</p>
                /* Only x left in stock message */
                if *in_stock < 10 {<p class="text-sm text-gray-500">{format!("Only {in_stock} left in stock")}</p>}
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
