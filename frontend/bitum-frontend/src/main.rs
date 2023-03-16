mod api;
mod components;
mod routes;

use std::rc::Rc;

use api::User;
use routes::Route;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{routes::switch};

#[derive(Clone, Debug, PartialEq)]
pub struct LoggedUserInfo {
    user: Option<User>,
}

impl Reducible for LoggedUserInfo {
    type Action = Option<User>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        LoggedUserInfo { user: action }.into()
    }
}

#[function_component]
fn App() -> Html {
    let logged_user_info = use_memo(|_| LoggedUserInfo { user: None }, ());

    html! {
        <div class="container">
            <ContextProvider<Rc<LoggedUserInfo>> context={logged_user_info}>
                <BrowserRouter>
                    <Switch<Route> render={switch}/>
                </BrowserRouter>
            </ContextProvider<Rc<LoggedUserInfo>>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
