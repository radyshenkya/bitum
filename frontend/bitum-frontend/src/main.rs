mod api;
mod components;
mod constants;
mod routes;
use routes::Route;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{components::ToastContainer, routes::switch};

pub const TOAST_CONTAINER_ID: &str = "toasts-container";

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <BrowserRouter>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
            <ToastContainer id={TOAST_CONTAINER_ID.to_string()}/>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
