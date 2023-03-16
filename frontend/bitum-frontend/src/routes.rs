pub mod chats;
pub mod index;
pub mod login;
pub mod register;

use chats::ChatsRoute;
use index::IndexRoute;
use login::LoginRoute;
use register::RegisterRoute;
use yew::{html, Html};
use yew_router::prelude::*;

use crate::components::LoginOrRedirect;

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
        Route::Chats => html! {
            <LoginOrRedirect>
                <ChatsRoute/>
            </LoginOrRedirect>
        },
        Route::Chat { id } => html! {<h1>{"Chat/"}{id}</h1>},
        Route::Login => html! {<LoginRoute />},
        Route::Register => html! {<RegisterRoute />},
        Route::NotFound => html! {<h1>{"Not found!"}</h1>},
    }
}
