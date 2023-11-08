use crate::Route;

use yew::{function_component, html, Html, Properties};
use yew_router::prelude::Link;



#[derive(Clone, Debug, PartialEq, Properties)]
pub struct HeaderProps {
    pub count: u32
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let cart_text = format!("cart: {}", props.count);

    html! {
        <div class="block">
            <header class="bg-gradient-to-r from-kiggyred to-kiggypink p-4 flex justify-between items-center">
                <div class="container mx-auto flex justify-between items-center">
                    <Link<Route> classes="text-2x1 text-white font-mono font-courier focus:outline-none focus:underline" to={Route::Gallery}>{"Kristen Rankin"}</Link<Route>>
                    <aside>
                        <Link<Route> classes="bg-kiggygreen text-white py-2 px-4 rounded-full brightness-100 transition ease duration-300 hover:brightness-80" to={Route::Cart}>{cart_text}</Link<Route>>
                    </aside>
                </div>
            </header>
            <hr class="relative m-0 p-0 bg-kiggygreen w-full h-1 border-0 bottom-0 left-0 z-0"/>
        </div>
    }
}
