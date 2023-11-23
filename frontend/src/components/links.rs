use web_sys::MouseEvent;
use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::{
    components::svg::{Instagram, LinkedIn, Mail, Twitter},
    utils::Palette,
};

const SVG_COLOR: Palette = Palette::Yellow;

#[derive(PartialEq, Clone)]
pub enum LinksSize {
    Large,
    Small,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LinksProps {
    pub class: AttrValue,
    pub size: LinksSize,
}

#[function_component(Links)]
pub fn links(LinksProps { class, size }: &LinksProps) -> Html {
    let (width, height, svg_class) = match size {
        LinksSize::Large => (24, 24, "w-8 h-8"),
        LinksSize::Small => (12, 12, "w-4 h-4"),
    };

    html! {
        <div {class}>
            <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                <Instagram {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class={svg_class} color={SVG_COLOR}/>
            </a>
            <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                <Twitter {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class={svg_class} color={SVG_COLOR}/>
            </a>
            <a href="www.instagram.com/k1ggy" target="_blank" rel="noopener noreferrer">
                <LinkedIn {width} {height} onclick={None::<Callback<MouseEvent>>} alt="instagram" class={svg_class} color={SVG_COLOR}/>
            </a>
            <a href="" target="_blank" rel="noopener noreferrer">
                <Mail {width} {height} onclick={None::<Callback<MouseEvent>>} alt="mail" class={svg_class} color={SVG_COLOR}/>
            </a>
        </div>
    }
}
