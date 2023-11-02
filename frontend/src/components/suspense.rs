use yew::{function_component, html, Html};

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {<p>{"loading . . ."}</p>}
}
