use crate::{utils::{Palette, KSVG}, components::svg::{Instagram, Twitter, LinkedIn}};
use web_sys::MouseEvent;
use yew::{function_component, html, Html, Callback};

#[function_component(Footer)]
pub fn footer() -> Html {

    let (width, height) = (24,24);
    html! {
        <footer class="bg-kiggygreen p-4 text-center mt-auto">
            <div class="flex justify-center space-x-4">
                <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                    <Instagram {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class="w-8 h-8" color={Palette::Pink}/>
                </a>
                <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                    <Twitter {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class="w-8 h-8" color={Palette::Pink}/>
                </a>
                <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                    <LinkedIn {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class="w-8 h-8" color={Palette::Pink}/>
                </a>
                <button class="text-white font-mono">{"contact me"}</button>
            </div>
        </footer>
    }
}
