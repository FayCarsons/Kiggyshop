use std::rc::Rc;

use crate::{
    error::{FEResult, FrontendError},
    utils::get_document,
};
use common::{from_str, item::FrontEndItem, log_debug, to_string, CartMap, ItemId, StockMap};
use serde::{Deserialize, Serialize};
use yew::{Properties, Reducible};

#[derive(Properties, PartialEq, Clone, Debug, Default)]
pub struct AppState {
    pub cart: Rc<Cart>,
    pub stock: Option<Rc<StockMap>>,
}

pub enum AppAction {
    LoadStock(Rc<StockMap>),
    UpdateCart(Rc<CartAction>),
}

impl AppState {
    pub fn update_cart(&self, action: Rc<CartAction>) -> FEResult<Self> {
        let mut cpy = self.cart.items.clone();

        match *action {
            CartAction::AddItem(id) | CartAction::IncItem(id) => {
                *cpy.entry(id).or_default() += 1;
            }
            CartAction::RemoveItem(id) => {
                cpy.remove_entry(&id);
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
            cart: Rc::new(new_cart),
        })
    }

    pub fn get_item(&self, id: &u32) -> Option<&FrontEndItem> {
        self.stock.as_ref().and_then(|stock| stock.get(id))
    }

    pub fn get_total(&self) -> f64 {
        self.cart.items.iter().fold(0., |total, (id, quantity)| {
            let item = self
                .stock
                .clone()
                .unwrap_or_default()
                .get(id)
                .cloned()
                .unwrap_or_default();
            let price = match item.kind.as_str() {
                "BigPrint" => 20.,
                "SmallPrint" => 7.,
                "Button" => 3.,
                _ => 0.,
            };
            total + price * *quantity as f64
        })
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
        self.items.keys().collect::<Vec<&ItemId>>() == other.items.keys().collect::<Vec<&ItemId>>()
            && self.items.values().collect::<Vec<&ItemId>>()
                == other.items.values().collect::<Vec<&ItemId>>()
    }
}

#[derive(Clone)]
pub enum CartAction {
    AddItem(ItemId),
    RemoveItem(ItemId),
    IncItem(ItemId),
    DecItem(ItemId),
}

impl Cart {
    pub fn new() -> Self {
        Self {
            items: CartMap::new(),
        }
    }

    pub fn count(&self) -> u32 {
        self.items.values().sum()
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
            return None;
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
                        let mut cart = cart;
                        cart.items.shrink_to_fit();
                        Some(cart)
                    } else {
                        log_debug!("{:?}", deser);
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

    pub fn delete() -> FEResult<()> {
        let document = get_document()?;

        document.set_cookie("")?;

        Ok(())
    }
}

impl From<CartMap> for Cart {
    fn from(value: CartMap) -> Self {
        Cart { items: value }
    }
}

impl Default for Cart {
    fn default() -> Self {
        Cart {
            items: CartMap::new(),
        }
    }
}

pub fn substring<'a>(source: &'a str, start: &'a str, end: &'a str) -> Option<&'a str> {
    let start_pos = source.find(start);

    if let Some(start_pos) = start_pos {
        let start_len = start.as_bytes().len();
        let source = &source[start_pos + start_len..];
        let end_pos = source.find(end);

        if let Some(end_pos) = end_pos {
            Some(&source[..end_pos + 1])
        } else {
            None
        }
    } else {
        None
    }
}
