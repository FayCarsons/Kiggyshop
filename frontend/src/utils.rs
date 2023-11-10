use super::error::{ErrorType, FEResult, FrontendError};

use gloo::{console::log, net::http::Request};
use web_sys::HtmlDocument;
use yew::{function_component, html, AttrValue, Html, Properties};

pub type Color = [u8; 3];

pub const PINK: Color = [255, 142, 173];
pub const RED: Color = [228, 67, 66];
pub const GREEN: Color = [125, 122, 25];

#[derive(Clone, Debug, PartialEq)]
pub enum Palette {
    Pink,
    Red,
    Green,
}

impl Palette {
    pub fn to_color(&self) -> Color {
        match self {
            Self::Pink => PINK,
            Self::Red => RED,
            Self::Green => GREEN,
        }
    }

    pub fn hex_color(&self) -> AttrValue {
        tailwind_color(self.to_color())
    }

    pub fn tailwind_class(&self) -> AttrValue {
        match self {
            Self::Pink => "kiggypink",
            Self::Red => "kiggyred",
            Self::Green => "kiggygreen",
        }
        .into()
    }
}

pub fn tailwind_color(color: Color) -> AttrValue {
    let [r, g, b] = color;
    format!("#{r:X}{g:X}{b:X}").to_lowercase().into()
}

pub fn make_colors() {
    [PINK, RED, GREEN]
        .iter()
        .zip(["kiggypink", "kiggyred", "kiggygreen"])
        .for_each(|(val, name)| {
            let s = format!("{name}: {}", tailwind_color(*val));
            log!(s)
        })
}

#[derive(Clone, PartialEq, Properties)]
pub struct SvgProps {
    pub src: AttrValue,
    pub class: AttrValue,
    pub alt: AttrValue,
    pub color: Palette,
}

#[function_component(KSVG)]
pub fn ksvg(
    SvgProps {
        src,
        color,
        class,
        alt,
    }: &SvgProps,
) -> Html {
    let color = color.hex_color();
    html! {
        <img {src} {class} style={format!("fill: {c}; color: {c};",c=color)} {alt}/>
    }
}

pub fn title_to_path(title: &str) -> AttrValue {
    let title = title.replace(" ", "");
    format!("/api/resources/images/{title}.png").into()
}

pub fn kind_to_price_category(kind: &str) -> (f32, AttrValue) {
    let price = match kind {
        "SmallPrint" => 7.,
        "Button" => 3.,
        _ => 20.,
    };
    (price, kind.to_owned().into())
}

pub fn get_document() -> FEResult<HtmlDocument> {
    use web_sys::wasm_bindgen::JsCast;

    gloo::utils::document()
        .dyn_into::<HtmlDocument>()
        .map_err(|e| FrontendError {
            _type: ErrorType::JsError,
            inner: AttrValue::from(e.to_string().as_string().unwrap_or_default()),
        })
}

pub fn get_quantity_element(quantity: &i32) -> Html {
    match quantity {
        0 => html! {<p class="text-kiggygreen mb-2">{"out of stock :/"}</p>},
        1..=10 => {
            html! {<p class="text-kiggygreen mb-2">{format!("only {quantity} available!")}</p>}
        }
        _ => html! {<></>},
    }
}

pub async fn fetch(url: &str) -> String {
    let resp = Request::get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    resp
}
