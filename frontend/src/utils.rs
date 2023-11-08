use super::error::{ErrorType, FEResult, FrontendError};

use gloo::console::log;
use web_sys::HtmlDocument;
use yew::AttrValue;

pub type Color = [u8; 3];

pub const PINK: Color = [255, 142, 173];
pub const RED: Color = [228, 67, 66];
pub const GREEN: Color = [125, 122, 25];

pub fn tailwind_color(color: Color) -> AttrValue {
    let [r, g, b] = color;
    format!("#{r:X}{g:X}{b:X}").to_lowercase().into()
}

pub fn make_colors() {
    [PINK, RED, GREEN].iter().zip(["kiggypink", "kiggyred", "kiggygreen"]).for_each(|(val, name)| {
        let s = format!("{name}: {}", tailwind_color(*val));
        log!(s)
    })
}

pub fn title_to_path(title: &str) -> AttrValue {
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
