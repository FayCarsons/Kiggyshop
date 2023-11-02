use crate::error::FrontendError;
use common::{from_str, to_string, CartMap, StockMap, item::Item};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::{Reducible, Properties};

use web_sys::HtmlDocument;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct AppState {
    pub cart: Cart,
    pub stock: Option<StockMap>
}

pub enum AppAction {
    LoadStock(StockMap),
    UpdateCart(CartAction)
}

impl AppState {
    pub fn update_cart(&self, action: CartAction) -> Self {
        let mut cpy = self.cart.items.clone();

        log!(format!("{:?}", self));

        match action {
            CartAction::AddItem(item) => {
                *cpy.entry(item).or_default() += 1;
            }
            CartAction::RemoveItem(item) => {
                cpy.remove_entry(&item);
            }
            CartAction::DecItem(item) => {
                if let Some(count) = cpy.get(&item) {
                    cpy.insert(item, count.saturating_sub(1));
                } else {
                    cpy.insert(item, 1);
                }
            }
        }
        let new_cart = Cart { items: cpy };
        new_cart.set_cookie().unwrap();
        AppState {stock: self.stock.clone(), cart: new_cart}
    }

    pub fn get_item(&self, id: i32) -> Option<&Item> {
        self.stock.as_ref().and_then(|stock| stock.get(&id))
    }
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AppAction::LoadStock(stock) => {AppState {cart: self.cart.clone(), stock: Some(stock)}}.into(),
            AppAction::UpdateCart(cart_action) => self.update_cart(cart_action).into()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Properties)]
pub struct Cart {
    pub items: CartMap,
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.items.keys().collect::<Vec<&i32>>() == other.items.keys().collect::<Vec<&i32>>()
            && self.items.values().copied().collect::<Vec<u32>>()
                == other.items.values().copied().collect::<Vec<u32>>()
    }
}

pub enum CartAction {
    AddItem(i32),
    RemoveItem(i32),
    DecItem(i32),
}

impl Cart {
    pub fn count(&self) -> u32 {
        self.items.values().into_iter().sum()
    }

    pub fn set_cookie(&self) -> Result<(), FrontendError> {
        use web_sys::wasm_bindgen::JsCast;

        let ser = to_string(self).map_err(|e| FrontendError::SerializationError(e.to_string()))?;
        let max_age = 60 * 60 * 12 * 31;
        let cookie = format!("cart={}, path=/, max-age={}", ser, max_age);

        let document = web_sys::window().unwrap().unchecked_into::<HtmlDocument>();

        document.set_cookie(&cookie).unwrap();
        log!(document);

        Ok(())
    }

    pub fn from_cookie() -> Option<Self> {
        use web_sys::wasm_bindgen::JsCast;
        let document = web_sys::window().unwrap().unchecked_into::<HtmlDocument>();

        match document.cookie() {
            Ok(s) => {
                log!("Got cookie: {}", &s);
                let de = from_str::<Cart>(&s).unwrap();
                Some(de)
            }
            Err(_) => None,
        }
    }
}
