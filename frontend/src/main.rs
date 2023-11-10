pub mod components;
pub mod context;
pub mod error;
pub mod hooks;
pub mod utils;

use context::{AppState, Cart};

use common::HashMap;
use components::{
    cart::CartPage, footer::Footer, gallery::Gallery, header::Header, product::ProductPage,
    suspense::Loading,
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
    match routes {
        Route::Cart => html! {
            <div class="flex flex-col min-h-screen">
                <Header count={Cart::from_cookie().unwrap_or_default().count()}/>
                <CartPage/>
                <Footer/>
            </div>
        },
        Route::Gallery => html! {<Gallery/>},
        Route::Product { id } => html! {
            <div class="flex flex-col min-h-screen">
                <Header count={Cart::from_cookie().unwrap_or_default().count()}/>
                <ProductPage id={id}/>
                <Footer/>
            </div>
        },
        Route::NotFound => html! {<h1>{"four owo four: not fownd :/"}</h1>},
    }
}
