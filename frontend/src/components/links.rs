use web_sys::MouseEvent;
use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::{
    components::svg::{Instagram, LinkedIn, Mail, Twitter},
    utils::Palette,
};

const SVG_COLOR: Palette = Palette::Yellow;

#[derive(Clone, PartialEq, Properties)]
pub struct LinksProps {
    pub class: AttrValue,
}

#[function_component(Links)]
pub fn links(LinksProps { class }: &LinksProps) -> Html {
    let (width, height) = (24, 24);
    html! {
        <div {class}>
            <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                <Instagram {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class="w-8 h-8" color={SVG_COLOR}/>
            </a>
            <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                <Twitter {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class="w-8 h-8" color={SVG_COLOR}/>
            </a>
            <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                <LinkedIn {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class="w-8 h-8" color={SVG_COLOR}/>
            </a>
            <a href="" target="_blank" rel="noopener norederrer">
                <Mail {width} {height} onclick={None::<Callback<MouseEvent>>} alt="mail" class="w-8 h-8" color={SVG_COLOR}/>
            </a>
        </div>
    }
}
