use common::item::Item;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, Html, Properties};

use crate::{
    context::{AppAction, CartAction},
    components::error::Error,
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
        <div class="cart-container">
            <div class="cart-items">
                {cart_items}
            </div>
            <button class="shop-btn" id="checkout-btn">{"checkout"}</button>
        </div>
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
        <div class="cart-item">
            <div class="cart-image">
                <img src={title_to_path(&item.title)} />
            </div>
            <div class="cart-details">
                <h3>{item.title.clone()}</h3>
                <p>{format!("${}", item.price())}</p>
                <p>{format!("qty: {qty}")}</p>
                <button class="remove-item" {onclick}>{"x"}</button>
            </div>

        </div>
    }
}
