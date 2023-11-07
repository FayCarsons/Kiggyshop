use crate::{error::{FEResult, FrontendError}, utils::get_document};
use common::{from_str, item::Item, to_string, CartMap, StockMap, log_debug};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::{Properties, Reducible};

use web_sys::HtmlDocument;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct AppState {
    pub cart: Cart,
    pub stock: Option<StockMap>,
}

pub enum AppAction {
    LoadStock(StockMap),
    UpdateCart(CartAction),
}

impl AppState {
    pub fn update_cart(&self, action: CartAction) -> FEResult<Self> {
        let mut cpy = self.cart.items.clone();

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
        new_cart.set_cookie()?;
        Ok(AppState {
            stock: self.stock.clone(),
            cart: new_cart,
        })
    }

    pub fn get_item(&self, id: i32) -> Option<&Item> {
        self.stock.as_ref().and_then(|stock| stock.get(&id))
    }
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AppAction::LoadStock(stock) => {
                AppState {
                    cart: self.cart.clone(),
                    stock: Some(stock),
                }
            }
            .into(),
            AppAction::UpdateCart(cart_action) => self.update_cart(cart_action).unwrap().into(),
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
            && self.items.values().collect::<Vec<&u32>>()
                == other.items.values().collect::<Vec<&u32>>()
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
        let ser = to_string(self)?;
        let max_age = 60 * 60 * 12 * 30 * 365;
        let cookie = format!("cart={}, path=/, max-age={}", ser, max_age);

        let document = get_document()?;

        document.set_cookie(&cookie)?;

        Ok(())
    }

    pub fn from_cookie() -> Option<Self> {
        let document = get_document().ok();
        let document = if let Some(document) = document {
            document 
        } else {
            log_debug!("Document is None");
            return None
        };

        

        let cookie = document.cookie();
        match cookie {
            Ok(ref s) => {
                log_debug!("Cookie: {:?}", document.cookie());
                let s = substring(s, "cart=", "},");
                if let Some(s) = s {
                    log_debug!("Substring: {}", s);
                    let deser = from_str::<Cart>(s);
                        if let Ok(cart) = deser {
                            Some(Cart::from(cart))
                        } else {
                            log_debug!("{:?}", deser );
                            None
                        }
                } else {
                    log_debug!("Cookie is None");
                    None
                }
            }
            _ => None,
        }
    }
}

impl From<CartMap> for Cart {
    fn from(value: CartMap) -> Self {
        Cart {items: value}
    }
}

pub fn substring<'a>(source: &'a str, start: &'a str, end: &'a str) -> Option<&'a str> {
    let start_pos = source.find(start);

    if let Some(start_pos) = start_pos {
        let start_len = start.as_bytes().len();
        let source = &source[start_pos+start_len..];
        let end_pos = source.find(end);
        
        if let Some(end_pos) = end_pos {
            Some(&source[..end_pos+1])
        } else {
            None
        }
    } else {
        None
    }
}