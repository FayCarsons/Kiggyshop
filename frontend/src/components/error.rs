use yew::{function_component, html, Html};

#[function_component(Error)]
pub fn error() -> Html {
    html! {
        <p>{"Sorry, we're having some trouble o_0 \n Be right back!"}</p>
    }
}
