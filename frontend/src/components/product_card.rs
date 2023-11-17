use common::item::Item;
use yew::{function_component, html, Callback, Html, Properties};

use crate::utils::{get_quantity_element, kind_to_price, title_to_path};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CardProps {
    pub product: Item,
    pub onclick: Callback<Item>,
}

#[function_component(ProductCard)]
pub fn product_card(props: &CardProps) -> Html {
    let Item {
        title,
        kind,
        quantity,
        ..
    } = props.product.clone();

    let price = kind_to_price(&kind);

    let onclick = {
        let props = props.clone();
        move |_| props.clone().onclick.emit(props.clone().product)
    };

    // Change from image darkening on hover to fade to white
    // or, last resort, blurring
    html! {
        <div class="max-w-full overflow-hidden shadow-lg transition duration-300 transform hover:scale-105 aspect-square">
            <img src={title_to_path(&title)} alt={title.clone()} class="w-full h-full object-cover transition duration-300 ease-in-out hover:scale-105" loading="lazy"/>
            <div class="absolute inset-0 flex flex-col items-center justify-center bg-white bg-opacity-0 transition duration-300 opacity-0 hover:opacity-100 hover:bg-opacity-75" onclick={onclick}>
                <h2 class="text-kiggypink text-4xl font-semibold mb-2 opacity-100">{title}</h2>
                {get_quantity_element(&quantity)}
                <p class="text-kiggygreen text-2xl opacity-100">{format!("${price}")}</p>
            </div>
        </div>
    }
}
