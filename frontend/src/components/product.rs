use common::item::Item;
use yew::{
    function_component, html, use_context, Callback, Html, MouseEvent, Properties,
};

use crate::{
    context::{AppAction, CartAction},
    utils::{kind_to_price_category, title_to_path},
    Context,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GalleryProps {
    pub product: Item,
    pub onclick: Callback<Item>,
}

#[function_component(GalleryProduct)]
pub fn gallery_product(props: &GalleryProps) -> Html {
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

    html! {
        <div class="max-w-full overflow-hidden shadow-lg transition duration-300 transform hover:scale-105 aspect-square">
            <img src={title_to_path(&title)} alt={title.clone()} class="w-full h-full object-cover transition duration-300 ease-in-out hover:scale-105" loading="lazy"/>
            <div class="absolute inset-0 flex flex-col items-center justify-center text-white bg-black bg-opacity-0 transition duration-300 opacity-0 hover:opacity-75 hover:bg-opacity-75" onclick={onclick}>
                <h2 class="text-white text-2xl font-semibold mb-2">{title}</h2>
                <p class="text-white mb-2">{format!("Qty: {quantity}")}</p>
                <p class="text-white">{format!("${price}")}</p>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FocusProps {
    pub id: i32,
}

#[function_component(ProductPage)]
pub fn product_page(props: &FocusProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let item = ctx.get_item(props.id).unwrap();

    let Item {
        title,
        kind,
        description,
        quantity,
        ..
    } = item;

    let (price, category) = kind_to_price_category(&kind);

    
    let onclick = {
        let id = props.id;
        let ctx = ctx.clone();
        move |_: MouseEvent| {
            ctx.dispatch(AppAction::UpdateCart(CartAction::AddItem(id)))
        }
    };

    html! {
        <div class="flex flex-col md:flex-row items-center" >
            <div class="md:w-1/2 p-4">
                <img src={title_to_path(&title)} alt={title.clone()} class="w-full h-auto object-cover rounded-lg"/>
            </div>
            <div class="md:w-1/2 p-4">
                <h1 class="text-3xl font-semibold mb-2">{title}</h1>
                <p class="text-gray-500 mb-2">{category}</p>
                <p class="text-gray-700 mb-4">{description}</p>
                <div class="flex items-center mb-4">
                    <span class="text-lg font-semibold text-gray-900 mr-2">{format!("${price}")}</span>
                    <span class="text-gray-500">{quantity}</span>
                </div>
                <button class="bg-gray-800 hover:bg-gray-700 text-white py-2 px-6 rounded-full focus:outline-none focus:ring focus:ring-gray-300" {onclick}>{"Add to cart"}</button>
            </div>
        </div>
    }
}
