use common::item::Item;

pub fn title_to_path(title: &str) -> String {
    format!("/api/resources/images/{title}.png")
}

pub fn kind_to_price_category(kind: &str) -> (f32, String) {
    let price = match kind {
        "SmallPrint" => 7.,
        "Button" => 3.,
        _ => 20.,
    };
    (price, kind.to_owned())
}
