use common::item::Item;
use yew::{
    function_component, html, use_context, use_reducer, Callback, Html, MouseEvent, Properties,
};

use crate::{
    context::{Cart, CartAction, AppAction},
    utils::{kind_to_price_category, title_to_path}, Context,
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
        <div class="gallery-product">
            <img src={title_to_path(&title)} loading="lazy"/>
            <div class="overlay" {onclick}>
                <h2>{title}</h2>
                <p>{format!("Qty: {quantity}")}</p>
                <p>{format!("${price}")}</p>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FocusProps {
    pub product: Item,
}

#[function_component(ProductPage)]
pub fn product_page(props: &FocusProps) -> Html {
    let Item {
        title,
        kind,
        description,
        quantity,
        ..
    } = props.product.clone();

    let (price, category) = kind_to_price_category(&kind);

    let ctx = use_context::<Context>().unwrap();
    let onclick: Callback<MouseEvent> = {
        let id = props.product.id;
        Callback::from(move |_: MouseEvent| ctx.dispatch(AppAction::UpdateCart(CartAction::AddItem(id))))
    };
    html! {
        <div class="product-details" >
            <div class="product-image">
                <img src={title_to_path(&title)} />
            </div>
            <div class="product-info">
                <h2>{title}</h2>
                <p>{category}</p>
                <p>{description}</p>
                <p>{format!("${price}")}</p>
                <p>{quantity}</p>
                <button class="shop-btn" id="add-to-cart-btn" {onclick}>{"Add to cart"}</button>
            </div>
        </div>
    }
}
