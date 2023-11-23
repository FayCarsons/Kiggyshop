use crate::{components::svg::Burger, utils::Palette, Route};

use gloo::console::log;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::{use_location, Link};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct HeaderProps {
    pub onclick: Callback<MouseEvent>,
    pub show_cart: bool,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let onclick = props.onclick.clone();
    let (width, height) = (24, 24);
    let color = Palette::Red;

    let location = use_location().unwrap();
    log!(location.path());

    html! {
        <header class="md:hidden">
            <div class="bg-gradient-to-r from-kiggyred to-kiggypink p-4 z-0">
                <div class="mx-auto flex justify-center">
                    <Link<Route> classes="text-4xl my-1 mx-auto text-white font-bubblegum focus:outline-none" to={Route::Gallery}>{"KiggyShop"}</Link<Route>>
                    if ! props.show_cart {
                        <aside>
                            <Burger {onclick} {width} {height} {color} alt="cart button" class="md:hidden"/>
                        </aside>
                    }
                </div>
            </div>
            <hr class="relative m-0 p-0 bg-kiggygreen w-full h-1 border-0 bottom-0 left-0 z-0"/>
        </header>
    }
}
