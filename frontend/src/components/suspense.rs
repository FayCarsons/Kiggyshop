use yew::{function_component, html, Html};

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center h-screen">
            <div class="animate-spin rounded-full h-16 w-16 border-t-4 border-kiggygreen border-solid"/>
            <p class="mt-4 text-kiggypink text-lg">{"Loading . . ."}</p>
        </div>
    }
}
