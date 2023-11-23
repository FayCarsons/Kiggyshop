use crate::components::links::{Links, LinksSize};
use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-kiggygreen p-4 text-center mt-auto md:hidden">
            <Links size={LinksSize::Large} class="flex justify-center space-x-4"/>
        </footer>
    }
}
