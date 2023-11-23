use common::{item::FrontEndItem, ItemId};
use yew::{
    function_component, html, use_context, use_state, Callback, Html, HtmlResult, MouseEvent,
    Properties, Suspense,
};

use crate::{
    components::{
        dropdown::CartDropdown, error::Error, footer::Footer, header::Header, suspense::Loading,
    },
    context::{AppAction, CartAction},
    hooks::use_stock,
    utils::{get_quantity_element, kind_to_price, title_to_path},
    Context,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProductPageProps {
    pub id: ItemId,
}

#[function_component(ProductPage)]
pub fn product_page(props: &ProductPageProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let fallback = html! {<Loading/>};

    let child = if ctx.stock.is_none() {
        html! {
            <Suspense {fallback}>
                <SuspendProduct id={props.id}/>
            </Suspense>
        }
    } else {
        html! {
            <SyncProduct id={props.id}/>
        }
    };

    let show_cart = use_state(|| false);
    let set_cart = {
        let show_cart = show_cart.clone();
        move |_| show_cart.set(!*show_cart)
    };

    let left_dropdown_class = "min-h-screen top-0 p-4 w-0 md:w-52 transition-all duration-300 ease-in-out bg-kiggygreen hidden md:flex md:flex-col items-start top-0 left-0";

    html! {
        <div class="relative flex min-h-screen bg-slate-50">
            <CartDropdown onclick={None::<Callback<MouseEvent>>} class={left_dropdown_class}/>
            <div class="flex flex-col min-h-screen">
                <Header show_cart={*show_cart} onclick={set_cart.clone()}/>
                {child}
                if ! *show_cart {<Footer/>}
            </div>

            if *show_cart {
                <CartDropdown onclick={set_cart} class={"bg-kiggygreen flex flex-col items-end top-0 right-0s md:hidden"}/>
            }
        </div>
    }
}

/*

*/

#[function_component(SuspendProduct)]
pub fn suspended_item(props: &ProductPageProps) -> HtmlResult {
    let ctx = use_context::<Context>().unwrap();
    let stock = use_stock()?;

    let err_case = html! {<Error/>};

    let item = if let Ok(stock) = stock {
        ctx.dispatch(AppAction::LoadStock(stock.clone()));
        let opt_item = stock.get(&props.id);
        if let Some(item) = opt_item.cloned() {
            item
        } else {
            return Ok(err_case);
        }
    } else {
        return Ok(err_case);
    };

    let onclick = get_add_cart_callback(ctx, props.id);

    Ok(html! {
        <Product item={item.clone()} {onclick}/>
    })
}

#[function_component(SyncProduct)]
pub fn sync_product(props: &ProductPageProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let item = ctx.get_item(&props.id).unwrap();
    let onclick = get_add_cart_callback(ctx.clone(), props.id);

    html! {<Product item={item.clone()} {onclick}/>}
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProductProps {
    item: FrontEndItem,
    onclick: Callback<MouseEvent>,
}

#[function_component(Product)]
pub fn product(ProductProps { item, onclick }: &ProductProps) -> Html {
    let FrontEndItem {
        id: _,
        title,
        kind,
        description,
        stock,
    } = item;

    let price = kind_to_price(kind);

    html! {
        <div class="flex flex-col items-center md:flex-row md:justify-center" >
            <div class="md:w-1/2 p-4 flex flex-col items-center justify-center">
                <img src={title_to_path(title)} alt={title.clone()} class="w-full h-auto object-cover lg"/>
            </div>
            <div class="md:w-1/2 p-4 text-center md:text-left">
                <h1 class="text-3xl font-semibold mb-2">{title}</h1>
                <p class="text-gray-700 mb-4">{description}</p>
                <div class="flex flex-col items-center justify-center md:items-start md:justify-start mb-4">
                    <p class="text-lg font-semibold text-gray-900 mr-2 md:left-0">{format!("${price}")}</p>
                    {get_quantity_element(stock)}
                </div>
                if *stock > 0 {
                    <button
                    class="bg-gradient-to-l from-yellow-300 to-kiggypink
                            brightness-100 text-white py-2 px-4 md:px-6 
                            rounded transiition duration-300 ease-in-out 
                            hover:brightness-90 focus:ring focus:ring-kiggypink" 
                    {onclick}>
                    {"Add to cart"}
                </button>
                }
            </div>
        </div>
    }
}

fn get_add_cart_callback(ctx: Context, item_id: ItemId) -> Callback<MouseEvent> {
    let ctx = ctx.clone();
    Callback::from(move |_: MouseEvent| {
        ctx.dispatch(AppAction::UpdateCart(CartAction::AddItem(item_id).into()))
    })
}
