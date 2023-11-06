use super::error::{ErrorType, FEResult, FrontendError};

use web_sys::HtmlDocument;
use yew::AttrValue;


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