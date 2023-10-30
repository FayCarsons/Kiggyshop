use common::item::Item;
use yew::{function_component, html, Callback, Html, Properties};

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

#[function_component(FocusProduct)]
pub fn focus_product(props: &FocusProps) -> Html {
    let Item {
        title,
        kind,
        description,
        quantity,
        ..
    } = props.product.clone();

    let (price, category) = kind_to_price_category(&kind);

    html! {
        <div class="product-details" >
            <div class="product-image">
                <img src={title_to_path(&title)} />
            </div>
            <div class="product-info">
                <h2>{title}</h2>
                <p>{category}</p>
                <p>{description}</p>
                <p>{price}</p>
                <p>{quantity}</p>
                <button class="add-to-cart-btn">{"Add to cart"}</button>
            </div>
        </div>
    }
}

fn title_to_path(title: &str) -> String {
    format!("/api/resources/images/{title}.png")
}

fn kind_to_price_category(kind: &str) -> (f32, String) {
    let price = match kind {
        "SmallPrint" => 7.,
        "Button" => 3.,
        _ => 20.,
    };
    (price, kind.to_owned())
}
