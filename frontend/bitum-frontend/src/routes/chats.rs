use bitum_frontend::get_random_color_image_url;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    api::get_chats,
    components::{ErrorMessage, Footer, Header, NewChatModalButton},
    constants::API_REFRESH_MILLIS,
    routes::Route,
};

#[derive(PartialEq, Properties)]
pub struct ChatsRouteProps {}

#[function_component]
pub fn ChatsRoute(props: &ChatsRouteProps) -> Html {
    let ChatsRouteProps {} = props;

    let error_message_state = use_state(|| Option::<String>::None);
    let chats_state = use_state(|| Vec::new());

    {
        let chats_state = chats_state.clone();
        let error_message_state = error_message_state.clone();

        spawn_local(async move {
            TimeoutFuture::new(API_REFRESH_MILLIS).await;

            let response = get_chats().await;

            if let Ok(response) = response {
                if response.ok {
                    chats_state.set(response.data.unwrap());
                } else {
                    error_message_state.set(Some("Не удалось получить список чатов".to_string()));
                }
            } else {
                error_message_state.set(Some("Сервер не отвечает".to_string()));
            }
        });
    }

    html! {
        <>
            <Header/>
            <h1 class="fw-medium fs-1">
                {"Чаты "}
                <NewChatModalButton redirect=true chat_name={"".to_string()}>
                    //<button class="btn btn-light btn-lg">
                            <i class="bi bi-plus-square-fill fs-1 p-3"></i>
                    //</button>
                </NewChatModalButton>
            </h1>
            <div class="row">
                {
                    for (*chats_state).iter().map(|chat| {
                        html! {
                            <div class="col-lg-4 col-md-12 p-2">
                                <Link<Route> classes="text-decoration-none d-flex grow-on-hover" to={Route::Chat {id: chat.id.clone()}}>
                                    <img class="rounded-start-2 border object-fit-scale" height=75px src={
                            if chat.icon.is_some() {
                                format!("/api/files/{}", (*chat).clone().icon.unwrap_or("null.png".to_string()))
                            } else {
                                get_random_color_image_url(chat.name.clone(), 75, 75)
                            }
                        } alt="Chat icon"/>
                                    <div class="rounded-end-2 text-overflow-ellipsis d-flex border border-start-0 bg-white flex-grow-1 align-items-center">
                                        <div class="p-3 fs-4 text-dark fw-normal">
                                            {chat.name.clone()}
                                        </div>
                                    </div>
                                </Link<Route>>
                            </div>
                        }
                    })
                }
            </div>
            <Footer/>
            if let Some(err) = (*error_message_state).clone() {
                <ErrorMessage
                    on_close={
                        let error_message_state = error_message_state.clone();

                        Callback::from(move |_| {
                            error_message_state.set(None);
                        })
                    }
                    value={err}
                />
            }
        </>
    }
}
