use common::item::Item;
use yew::{function_component, Properties, Callback, Html, html};

use crate::utils::{kind_to_price_category, title_to_path, get_quantity_element};

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

    let (price, _) = kind_to_price_category(&kind);

    let onclick = {
        let props = props.clone();
        move |_| props.clone().onclick.emit(props.clone().product)
    };

    // Change from image darkening on hover to fade to white 
    // or, last resort, blurring
    html! {
        <div class="max-w-full overflow-hidden shadow-lg transition duration-300 transform hover:scale-105 aspect-square">
            <img src={title_to_path(&title)} alt={title.clone()} class="w-full h-full object-cover transition duration-300 ease-in-out hover:scale-105" loading="lazy"/>
            <div class="absolute inset-0 flex flex-col items-center justify-center text-white bg-black bg-opacity-0 transition duration-300 opacity-0 hover:opacity-75 hover:bg-opacity-75" onclick={onclick}>
                <h2 class="text-kiggyred text-2xl font-semibold mb-2">{title}</h2>
                {get_quantity_element(&quantity)}
                <p class="text-kiggypink">{format!("${price}")}</p>
            </div>
        </div>
    }
}