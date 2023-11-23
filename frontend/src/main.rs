pub mod components;
pub mod context;
pub mod error;
pub mod hooks;
pub mod utils;

use std::rc::Rc;

use context::{AppState, Cart};

use common::{HashMap, ItemId};
use components::{cart_page::CartPage, gallery::Gallery, product::ProductPage, suspense::Loading};
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

pub type Context = UseReducerHandle<AppState>;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Gallery,
    #[at("/product/:id")]
    Product { id: ItemId },
    #[at("/cart")]
    Cart,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    let ctx = use_reducer(|| {
        let cart = if let Some(cart) = Cart::from_cookie() {
            cart
        } else {
            Cart {
                items: HashMap::new(),
            }
        };
        let cart = Rc::new(cart);
        AppState { cart, stock: None }
    });

    let fallback = html! {<Loading />};

    let html = html! {
       <div>
            <ContextProvider<Context> context={ctx}>
                <Suspense fallback={fallback}>
                    <BrowserRouter>
                        <Switch<Route> render={switch}/>
                    </BrowserRouter>
                </Suspense>
            </ContextProvider<Context>>
       </div>
    };

    html
}

fn main() {
    log!("Main is rendering");
    yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Gallery => html! {<Gallery/>},
        Route::Product { id } => html! {
            <ProductPage {id}/>
        },
        Route::Cart => html! {<CartPage/>},
        Route::About => html! {<h1>{"About Page"}</h1>},
        Route::NotFound => html! {<h1>{"four owo four: not fownd :/"}</h1>},
    }
}
