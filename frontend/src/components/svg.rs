use yew::{function_component, html, AttrValue, Html, Properties};

use crate::utils::Palette;

#[derive(Clone, PartialEq, Properties)]
pub struct SvgProps {
    pub class: AttrValue,
    pub alt: AttrValue,
    pub color: Palette,
    pub width: u8,
    pub height: u8,
}

#[function_component(Instagram)]
pub fn instagram(
    SvgProps { class, alt, color, width, height }: &SvgProps,
) -> Html {
    let children = html!{<><rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect>
    <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path>
    <line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line></>};

    html!{<SvgWrapper {children} {class} {alt} fill={None} stroke={Some(color.clone())} {width} {height}/>}
}

#[function_component(Twitter)]
pub fn twitter(
    SvgProps {
        class,
        alt,
        color,
        width,
        height,
    }: &SvgProps,
) -> Html {
    let children = html!{<path
        d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z"></path>};

    html! {
            <SvgWrapper {children} {class} {alt} fill={None} stroke={Some(color.clone())} {width} {height}/>
    }
}

#[function_component(LinkedIn)]
pub fn linkedin(
    SvgProps {
        class,
        alt,
        color,
        width,
        height,
    }: &SvgProps,
) -> Html {
    let children = html!{<><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
    <rect x="2" y="9" width="4" height="12"></rect>
    <circle cx="4" cy="4" r="2"></circle></>};

    html! {
        <SvgWrapper {children} {class} {alt} fill={None} stroke={Some(color.clone())} {width} {height}/>
    }
}

#[function_component(Burger)]
pub fn burger(
    SvgProps {
        class,
        alt,
        color,
        width,
        height,
    }: &SvgProps,
) -> Html {
    let children = html!{<><line x1="21" y1="10" x2="3" y2="10"></line>
    <line x1="21" y1="6" x2="3" y2="6"></line>
    <line x1="21" y1="14" x2="3" y2="14"></line>
    <line x1="21" y1="18" x2="3" y2="18"></line></>};

    html! {
        <SvgWrapper {children} {class} {alt} fill={None} stroke={Some(color.clone())} {width} {height}/>
    }
}

#[function_component(ChevronUp)]
pub fn chevron_up(
    SvgProps {
        class,
        alt,
        color,
        width,
        height,
    }: &SvgProps,
) -> Html {
    let children = html!{<><polyline points="17 11 12 6 7 11"></polyline>
    <polyline points="17 18 12 13 7 18"></polyline></>};


    html! {
        <SvgWrapper {children} {class} {alt} fill={None} stroke={Some(color.clone())} {width} {height}/>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct SvgWrapperProps {
    width: u8,
    height: u8,
    fill: Option<Palette>,
    stroke: Option<Palette>,
    class: AttrValue,
    alt: AttrValue,
    children: Html,
}

#[function_component(SvgWrapper)]
fn svg_wrapper(
    SvgWrapperProps {
        width,
        height,
        fill,
        stroke,
        class,
        alt,
        children,
    }: &SvgWrapperProps,
) -> Html {
    let fill = match fill {
        Some(color) => color.hex_color(),
        None => "none".into() 
    };

    let stroke = match stroke {
        Some(color) => color.hex_color(),
        None => "none".into() 
    };

    let (width, height): (&AttrValue, &AttrValue) =
        (&width.to_string().into(), &height.to_string().into());

    html! {<svg xmlns="http://www.w3.org/2000/svg" {width} {height} viewBox={format!("0 0 {width} {height}")} fill={fill}
    stroke={stroke} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
    {class} {alt}>{children.clone()}</svg>  }
}
