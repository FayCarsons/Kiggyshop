pub mod components;
pub mod context;
pub mod error;
pub mod hooks;
pub mod utils;

use context::{AppState, Cart};

use common::HashMap;
use components::{
    cart::{CartPage, CartDropdown}, footer::Footer, gallery::Gallery, header::Header, product::ProductPage,
    suspense::Loading, error::Error,
};
use utils::make_colors;
use yew::prelude::*;
use yew_router::prelude::*;

pub type Context = UseReducerHandle<AppState>;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Gallery,
    #[at("/product/:id")]
    Product { id: i32 },
    #[at("/cart")]
    Cart,
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
    yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
    let onclick = Callback::from(|_:MouseEvent| {});

    match routes {
        Route::Cart => html! {
            <Error/>
        },
        Route::Gallery => html! {<Gallery/>},
        Route::Product { id } => html! {
            <ProductPage {id}/>
        },
        Route::NotFound => html! {<h1>{"four owo four: not fownd :/"}</h1>},
    }
}
