use yew::{function_component, html, Html};

#[function_component(BackendError)]
pub fn backend_error() -> Html {
    html! {
    <p>{"Sorry, we're having some trouble o_0 \n Be right back!"}</p>
    }
}
