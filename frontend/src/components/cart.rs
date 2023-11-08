use common::item::Item;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, Html, Properties};

use crate::{
    components::error::Error,
    context::{AppAction, CartAction},
    utils::title_to_path,
    Context,
};

#[function_component(CartPage)]
pub fn cart() -> Html {
    let ctx = use_context::<Context>().unwrap();

    let cart_items = ctx
        .cart
        .items
        .iter()
        .map(|(item_id, qty)| {
            let item = ctx.get_item(*item_id);
            let item = match item {
                Some(item) => item,
                None => return html! {<Error/>},
            };
            html! {<CartItem item={item.clone()}qty={*qty}/>}
        })
        .collect::<Html>();

    html! {
        <section class="bg-gray-100 py-8">
            <div class="container mx-auto">
                <article class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    {cart_items}
                </article>
                <button class="shop-btn" id="checkout-btn">{"checkout"}</button>
            </div>
        </section>
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct CartItemProps {
    pub item: Item,
    pub qty: u32,
}

#[function_component(CartItem)]
pub fn cart_item(props: &CartItemProps) -> Html {
    let CartItemProps { item, qty } = props;

    let id = item.id.clone();

    let ctx = use_context::<Context>().unwrap();

    let onclick =
        { move |_: MouseEvent| ctx.dispatch(AppAction::UpdateCart(CartAction::RemoveItem(id))) };

    html! {
        <div class="bg-white rounded-lg p-6 flex items-center space-x-4 shadow-md">
            <img class="w-20 h-20 object-cover rounded" src={title_to_path(&item.title)} />
            <div>
                <h2 class="text-lg font-semibold">{item.title.clone()}</h2>
                <p class="text-gray-500">{format!("${}", item.price())}</p>
                <p class="text-gray-500">{format!("qty: {qty}")}</p>
                <button class="text-red-500 hover:text-red-700 focus:outline-none" {onclick}>{"x"}</button>
            </div>
        </div>
    }
}
