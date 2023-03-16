mod api;
mod components;
mod routes;
use routes::Route;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::routes::switch;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <BrowserRouter>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
