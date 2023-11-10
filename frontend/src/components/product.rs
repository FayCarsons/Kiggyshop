use common::{from_str, item::Item, StockMap};
use yew::{
    function_component, html, platform::spawn_local, suspense::SuspensionResult, use_context,
    Callback, Html, HtmlResult, MouseEvent, Properties, Suspense,
};

use crate::{
    components::{error::Error, suspense::Loading},
    context::{AppAction, CartAction},
    hooks::use_item,
    utils::{fetch, get_quantity_element, kind_to_price_category, title_to_path},
    Context,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProductPageProps {
    pub id: i32,
}

#[function_component(ProductPage)]
pub fn product_page(props: &ProductPageProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let fallback = html! {<Loading/>};

    if ctx.stock.is_none() || !ctx.stock.as_ref().unwrap().contains_key(&props.id) {
        html! {
            <Suspense {fallback}>
                <SuspendProduct id={props.id}/>
            </Suspense>
        }
    } else {
        html! {
            <SyncProduct id={props.id}/>
        }
    }
}

#[function_component(SuspendProduct)]
pub fn suspended_item(props: &ProductPageProps) -> HtmlResult {
    let ctx = use_context::<Context>().unwrap();
    let item = use_item(&props.id)?;

    if item.is_err() {
        return Ok(html! {<Error/>});
    }

    let onclick = get_onclick(ctx, props.id);

    Ok(html! {
        <Product item={item.unwrap()} {onclick}/>
    })
}

#[function_component(SyncProduct)]
pub fn sync_product(props: &ProductPageProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let item = ctx.get_item(props.id).unwrap();
    let onclick = get_onclick(ctx.clone(), props.id);

    html! {<Product item={item.clone()} {onclick}/>}
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProductProps {
    item: Item,
    onclick: Callback<MouseEvent>,
}

#[function_component(Product)]
pub fn product(ProductProps { item, onclick }: &ProductProps) -> Html {
    let Item {
        title,
        kind,
        description,
        quantity,
        ..
    } = item;

    let (price, _) = kind_to_price_category(kind);

    html! {
        <div class="flex flex-col items-center md:flex-row md:justify-center" >
            <div class="md:w-1/2 p-4 flex flex-col items-center justify-center">
                <img src={title_to_path(&title)} alt={title.clone()} class="w-full h-auto object-cover lg"/>
            </div>
            <div class="md:w-1/2 p-4 text-center md:text-left">
                <h1 class="text-3xl font-semibold mb-2">{title}</h1>
                <p class="text-gray-700 mb-4">{description}</p>
                <div class="flex items-center justify-center mb-4">
                    <span class="text-lg font-semibold text-gray-900 mr-2">{format!("${price}")}</span>
                    {get_quantity_element(&quantity)}
                </div>
                <button 
                    class="bg-gradient-to-l from-yellow-300 to-kiggypink 
                            brightness-100 text-white py-2 px-4 md:px-6 
                            rounded transiition duration-300 ease-in-out 
                            hover:brightness-90 focus:ring focus:ring-kiggypink" 
                    {onclick}>
                    {"Add to cart"}
                </button>
            </div>
        </div>
    }
}

fn get_onclick(ctx: Context, item_id: i32) -> Callback<MouseEvent> {
    let ctx = ctx.clone();
    Callback::from(move |_: MouseEvent| {
        ctx.dispatch(AppAction::UpdateCart(CartAction::AddItem(item_id)))
    })
}
