use gloo::console::log;
use yew::{function_component, html, Html};

use crate::{context::Cart, utils::title_to_path};

#[function_component(OrderCompleted)]
pub fn order_completed() -> Html {
    match Cart::delete() {
        Ok(_) => (),
        Err(e) => log!(e.to_string()),
    };

    html! {
       <div class="w-full h-full">
        <img src={title_to_path("order_completed")} class="object-cover"/>
       </div>
    }
}
