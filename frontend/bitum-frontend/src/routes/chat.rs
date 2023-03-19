use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::{get_chat, get_messages, Chat, ChatMessage as ApiChatMessage},
    components::{ChatMessage, ErrorMessage, Footer, Header},
    constants::API_REFRESH_MILLIS,
};

#[derive(PartialEq, Properties)]
pub struct ChatRouteProps {
    pub chat_id: i32,
}

#[function_component]
pub fn ChatRoute(props: &ChatRouteProps) -> Html {
    let ChatRouteProps { chat_id } = props;
    let error_message_state = use_state(|| Option::<String>::None);
    let chat_state = use_state(|| Option::<Chat>::None);
    let messages_state = use_state(|| Vec::<ApiChatMessage>::new());

    {
        let error_message_state = error_message_state.clone();
        let chat_id = chat_id.clone();
        let chat_state = chat_state.clone();

        if chat_state.is_none() {
            spawn_local(async move {
                let error_message_state = error_message_state.clone();
                let chat_id = chat_id.clone();
                let chat_state = chat_state.clone();

                let response = get_chat(chat_id).await;

                if let Ok(response) = response {
                    if response.ok {
                        chat_state.set(response.data);
                    } else {
                        error_message_state
                            .set(Some("Не удалось получить информацию о чате".to_string()));
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }
            })
        }
    };

    {
        let error_message_state = error_message_state.clone();
        let chat_id = chat_id.clone();
        let messages_state = messages_state.clone();

        spawn_local(async move {
            let error_message_state = error_message_state.clone();
            let chat_id = chat_id.clone();
            let messages_state = messages_state.clone();

            TimeoutFuture::new(API_REFRESH_MILLIS).await;

            let response = get_messages(chat_id, 40, 0).await;

            if let Ok(response) = response {
                if response.ok {
                    messages_state.set(response.data.unwrap());
                } else {
                    error_message_state.set(Some("Не удалось получить сообщения".to_string()));
                }
            } else {
                error_message_state.set(Some("Сервер не отвечает".to_string()));
            }
        });
    }

    html! {
        <>
            <Header/>
            if let Some(chat) = (*chat_state).clone() {
                <h1 class="fw-medium fs-1">
                    <img width=60px class="rounded-3" src={format!("/api/files/{}", chat.icon.unwrap_or("null.png".to_string()))} />
                    <span class="p-3">
                        {chat.name}
                    </span>
                </h1>
            }

            <div class="row">
                <div class="col-lg-8 col-md-12 gy-3">
                    <form class="row gx-1">
                        <div class="col-lg-10 col-md-12 p-1">
                            <textarea type="type" placeholder="Сообщение" class="form-control" />
                        </div>
                        <div class="col-lg-2 col-md-12 p-0">
                            <button type="submit" class="btn btn-outline-success">{"Отправить"}</button>
                        </div>
                    </form>
                    <div class="col-12 overflow-x-scroll">
                        { for (*messages_state).iter().map(|message| html! {
                            <ChatMessage message={message.clone()}  />
                        }) }
                    </div>
                </div>
                <div class="col-lg-4 col-md-12">
                    {"aboba"}
                </div>
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
