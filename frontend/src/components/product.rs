use common::{item::Item, from_str, StockMap};
use yew::{function_component, html, use_context, Html, MouseEvent, Properties, platform::spawn_local};

use crate::{
    context::{AppAction, CartAction},
    utils::{kind_to_price_category, title_to_path, fetch, get_quantity_element},
    Context,
};



#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FocusProps {
    pub id: i32,
}

#[function_component(ProductPage)]
pub fn product_page(props: &FocusProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let item = match ctx.get_item(props.id) {
        Some(i) => i.clone(),
        None => {
            let id = props.id.clone();
            let url = format!("/api/stock/get_single/{}", props.id);
            let link = ctx.clone();
            spawn_local(async move {
                let s = fetch(&url).await;
                let de = from_str::<Item>(&s).unwrap();
                link.dispatch(AppAction::LoadStock(StockMap::from([(id, de)])))
            });
            ctx.get_item(props.id).unwrap().clone()
        }
    };

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
        move |_: MouseEvent| ctx.dispatch(AppAction::UpdateCart(CartAction::AddItem(id)))
    };

    html! {
        <div class="flex flex-col items-center md:flex-row md:justify-center" >
            <div class="md:w-1/2 p-4 flex flex-col items-center justify-center">
                <img src={title_to_path(&title)} alt={title.clone()} class="w-full h-auto object-cover lg"/>
            </div>
            <div class="md:w-1/2 p-4 text-center md:text-left">
                <h1 class="text-3xl font-semibold mb-2">{title}</h1>
                <p class="text-gray-500 mb-2">{category}</p>
                <p class="text-gray-700 mb-4">{description}</p>
                <div class="flex items-center justify-center mb-4">
                    <span class="text-lg font-semibold text-gray-900 mr-2">{format!("${price}")}</span>
                    {get_quantity_element(&quantity)}
                </div>
                <button class="bg-gradient-to-r from-yellow-100 to-kiggypink hover:brightness-90 text-white py-2 px-4 md:px-6 rounded focus:ring focus:ring-gray-300" {onclick}>{"Add to cart"}</button>
            </div>
        </div>
    }
}
