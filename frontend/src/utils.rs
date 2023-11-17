use super::error::{ErrorType, FEResult, FrontendError};

use web_sys::HtmlDocument;
use yew::{html, AttrValue, Html};

pub type Color = [u8; 3];

pub const PINK: Color = [255, 142, 173];
pub const RED: Color = [228, 67, 66];
pub const GREEN: Color = [125, 122, 25];
pub const WHITE: Color = [255, 255, 255];
pub const BLACK: Color = [0, 0, 0];
pub const YELLOW: Color = [255, 241, 118];

#[derive(Clone, Debug, PartialEq)]
pub enum Palette {
    Pink,
    Red,
    Green,
    White,
    Black,
    Yellow,
}

impl Palette {
    pub fn to_color(&self) -> Color {
        match self {
            Self::Pink => PINK,
            Self::Red => RED,
            Self::Green => GREEN,
            Self::White => WHITE,
            Self::Black => BLACK,
            Self::Yellow => YELLOW,
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
            Self::White => "white",
            Self::Black => "black",
            Self::Yellow => "yellow-300",
        }
        .into()
    }
}

pub fn tailwind_color(color: Color) -> AttrValue {
    let [r, g, b] = color;
    format!("#{r:X}{g:X}{b:X}").to_lowercase().into()
}

pub fn title_to_path(title: &str) -> AttrValue {
    let title = title.replace(' ', "");
    format!("/api/resources/images/{title}.png").into()
}

pub fn kind_to_price(kind: &str) -> AttrValue {
    let price = match kind {
        "SmallPrint" => 7.,
        "Button" => 3.,
        _ => 20.,
    };

    price.to_string().into()
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
