use crate::{Route, components::svg::Burger, utils::Palette};

use web_sys::MouseEvent;
use yew::{function_component, html, Html, Properties, Callback};
use yew_router::prelude::Link;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct HeaderProps {
    pub onclick: Callback<MouseEvent>
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    //let cart_text = format!("cart: {}", props.count);
    
    html! {
        <header class="inline">
            <div class="bg-gradient-to-r from-kiggyred to-kiggypink p-4 flex justify-between items-center">
                <div class="container mx-auto flex justify-between items-center">
                    <Link<Route> classes="text-2x1 text-white font-mono font-courier focus:outline-none focus:underline" to={Route::Gallery}>{"Kristen Rankin"}</Link<Route>>
                    <aside>
                        <Burger onclick={props.onclick.clone()} width={24} height={24} alt="cart button" color={Palette::Green} class="lg:hidden"/>
                    </aside>
                </div>
            </div>
            <hr class="relative m-0 p-0 bg-kiggygreen w-full h-1 border-0 bottom-0 left-0 z-0"/>
        </header>
    }
}
