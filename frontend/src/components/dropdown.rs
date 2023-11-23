use std::rc::Rc;

use common::{item::FrontEndItem, ItemId, Quantity};
use web_sys::MouseEvent;
use yew::{
    function_component, html, use_context, AttrValue, Callback, Html,
    Properties,
};
use yew_router::prelude::{use_navigator, Link};

use crate::{
    components::{
        error::Error,
        links::{Links, LinksSize},
        svg::Burger,
    },
    context::{AppAction, CartAction},
    utils::{kind_to_price, title_to_path, Palette, checkout},
    Context, Route,
};

pub const BASE_DROPDOWN_CLASS: &str = "";
const DEFAULT_DROPDOWN_CLASS: &str = "w-52";

// OLD CART-DROPDOWN

#[derive(Clone, PartialEq, Properties)]
pub struct DropDownProps {
    pub onclick: Option<Callback<MouseEvent>>,
    pub class: AttrValue
}

#[function_component(CartDropdown)]
pub fn cart_dropdown(DropDownProps { onclick , class}: &DropDownProps) -> Html {
    let ctx = use_context::<Context>().unwrap();
    let total = ctx.get_total();

    // Sort products by ID so they don't re-order on re-render
    // Can't find aa way to do this w/o allocating twice
    let mut products: Vec<(ItemId, Quantity)> = ctx.cart.items.clone().into_iter().collect();
    products.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let products = products
        .iter()
        .map(|(id, quantity)| {
            if *quantity == 0 {
                ctx.dispatch(AppAction::UpdateCart(Rc::new(CartAction::RemoveItem(*id))));
                return html! {<></>};
            }
            let item = ctx.get_item(id);
            match item {
                Some(item) => html! {<DropdownItem key={*id} item={item.clone()} {quantity}/>},
                None => html! {<Error/>},
            }
        })
        .collect::<Html>();

    let checkout = checkout(ctx.cart.clone());

    html! {
        <div {class}>
            if onclick.is_some() {
                <Burger
                    onclick={onclick.clone().unwrap_or_default()}
                    width={24}
                    height={24}
                    alt="cart button"
                    color={Palette::Green}
                    class="absolute top-4 right-4 md:hidden"
                />
            }
            <h2 class="text-xl font-bold font-mono mb-4">{"cart"}</h2>

            <div class="mb-4 space-y-4">
                {products}
            </div>

            <div class="border-t border-kiggypink pt-2 mb-4">
                <p class="text-sm font-mono font-semiboold">{format!("total: ${}", total)}</p>
                <Link<Route> to={Route::Cart}>{"TEST CART"}</Link<Route>>
            </div>

            <button onclick={checkout} class="bg-kiggypink text-white font-mono py-2 px-4 rounded hover:brightness-90">
                {"checkout"}
            </button>

            <Links size={LinksSize::Large} class="absolute bottom-0 md:left-0 mt-auto mx-auto space-x-2 p-2 flex justify-center items-center"/>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CartItemProps {
    pub item: FrontEndItem,
    pub quantity: Quantity,
}

#[function_component(DropdownItem)]
fn dropdown_item(CartItemProps { item, quantity }: &CartItemProps) -> Html {
    let FrontEndItem {
        id,
        title,
        kind,
        description,
        stock: _,
    } = item;
    let price = kind_to_price(kind);

    let navigator = use_navigator().unwrap();

    let product_callback = {
        let id = *id;
        let navigator = navigator.clone();
        move |_: MouseEvent| navigator.push(&Route::Product { id })
    };

    let ctx = use_context::<Context>().unwrap();

    let make_cart_callback = {
        move |action: CartAction| {
            let ctx = ctx.clone();
            let action = Rc::new(action);
            Callback::from(move |_: MouseEvent| ctx.dispatch(AppAction::UpdateCart(action.clone())))
        }
    };

    html! {
        <div class="flex items-start mb-4">
            // Product images
            <img
                onclick={product_callback.clone()}
                src={title_to_path(title)}
                alt={AttrValue::from(description.clone())}
                class="w-24 h-24 object-cover aspect-square mr-4 rounded"
            />

            // Details
            <div class="flex flex-col">
                <div class="flex items-center">
                    <p onclick={product_callback} class="text-sm font-mono font-semibold mb-1">
                    {title}
                    </p>
                    <button
                        label="remove button"
                        onclick={make_cart_callback(CartAction::RemoveItem(*id))}
                        class="text-kiggyred text-sm font-mono mx-auto focus:outline-none">
                        {"x"}
                    </button>
                </div>

                <p class="text-xs font-mono">
                    {format!("${price}")}
                </p>

                <div class="flex items-center space-x-1 bottom-0 mb-2">
                    <button
                        onclick={make_cart_callback(CartAction::DecItem(*id))}
                        class="text-gray-500 focus:outline-none">
                        {"-"}
                    </button>
                    <input
                        type="text"
                        value={quantity.to_string()}
                        class="w-8 h-6 m-1 text-center text-xs border rounded focus:outline-none"
                        readonly={true}
                    />
                    <button
                        onclick={make_cart_callback(CartAction::IncItem(*id))}
                        class="text-gray-500 focus:outline-none">
                        {"+"}
                    </button>
                </div>
            </div>
        </div>
    }
}
