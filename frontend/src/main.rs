pub mod components;
pub mod error;
pub mod hooks;
use common::Stock;
use components::{home::Home, suspense::Loading};
use std::sync::OnceLock;
use yew::prelude::*;
pub static STOCK: OnceLock<Stock> = OnceLock::new();

#[function_component(App)]
fn app() -> Html {
    let fallback = html! {<Loading />};

    let html = html! {
       <div>
            <Suspense fallback={fallback}>
                <Home/>
            </Suspense>
       </div>
    };

    html
}

fn main() {
    yew::Renderer::<App>::new().render();
}
