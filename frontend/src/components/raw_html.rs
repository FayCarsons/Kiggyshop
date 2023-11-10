use yew::{function_component, Properties, AttrValue, Html};

use crate::utils::get_document;


#[derive(Clone, PartialEq, Properties)]
pub struct RawHtmlProps {
    pub inner: AttrValue
}

#[function_component(RawHtml)]
pub fn raw_html(props: &RawHtmlProps) -> Html {
    let div = get_document().unwrap().create_element("div").unwrap();
    div.set_inner_html(&props.inner.clone());

    Html::VRef(div.into())
}