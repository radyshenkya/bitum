pub mod index;

use index::IndexRoute;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/chats")]
    Chats,
    #[at("/chat/:id")]
    Chat { id: i32 },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {<IndexRoute/>},
        Route::Chats => html! {<h1>{"Chats page!"}</h1>},
        Route::Chat { id } => html! {<h1>{"Chat/"}{id}</h1>},
        Route::Login => html! {<h1>{"Login page!"}</h1>},
        Route::Register => html! {<h1>{"Register page!"}</h1>},
        Route::NotFound => html! {<h1>{"Not found!"}</h1>},
    }
}
