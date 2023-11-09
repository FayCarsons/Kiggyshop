use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-kiggygreen p-4 text-center">
            <div class="flex justify-center space-x-4">
                <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                    <img src="/api/resources/icons/instagram.svg" alt="instagram" class="w-8 h-8"/>
                </a>
                <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                    <img src="/api/resources/icons/twitter.svg" alt="twitter" class="w-8 h-8"/>
                </a>
                <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                    <img src="/api/resources/icons/linkedin.svg" alt="linkedin" class="w-8 h-8"/>
                </a>
                <button class="text-white font-mono">{"contact me"}</button>
            </div>
        </footer>
    }
}
