pub mod components;
pub mod error;
pub mod hooks;
use components::{home::Home, suspense::Loading};
use std::sync::OnceLock;
use yew::prelude::*;

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
