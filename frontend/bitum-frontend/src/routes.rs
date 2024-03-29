pub mod chat;
pub mod chats;
pub mod index;
pub mod login;
pub mod my_bots;
pub mod register;

use chat::ChatRoute;
use chats::ChatsRoute;
use index::IndexRoute;
use login::LoginRoute;
use my_bots::MyBotsRoute;
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
    #[at("/my_bots")]
    MyBots,
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
        Route::Chat { id } => html! {
            <LoginOrRedirect>
                <ChatRoute chat_id={id} />
            </LoginOrRedirect>
        },
        Route::Login => html! {<LoginRoute />},
        Route::Register => html! {<RegisterRoute />},
        Route::MyBots => html! {
            <LoginOrRedirect>
                <MyBotsRoute />
            </LoginOrRedirect>
        },
        Route::NotFound => html! {<h1>{"Not found!"}</h1>},
    }
}
