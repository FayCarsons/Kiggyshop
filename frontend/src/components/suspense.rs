use yew::{function_component, html, Html};

#[function_component(Fallback)]
pub fn fallback() -> Html {
    html! {<></>}
}

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {<p>{"loading . . ."}</p>}
}
