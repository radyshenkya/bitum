use bitum_frontend::get_random_color_image_url;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::{
    api::{
        get_chat, get_messages, send_message, Chat, ChatMessage as ApiChatMessage,
        SendMessageRequest,
    },
    components::{ChatMessage, ErrorMessage, Header},
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
    let message_input_node = use_node_ref();
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

    let on_submit = {
        let message_input_node = message_input_node.clone();
        let error_message_state = error_message_state.clone();
        let chat_id = chat_id.clone();

        Callback::from(move |_: MouseEvent| {
            let message_input_node = message_input_node.clone();
            let error_message_state = error_message_state.clone();
            let chat_id = chat_id.clone();

            spawn_local(async move {
                let message_input_node = message_input_node
                    .clone()
                    .cast::<HtmlTextAreaElement>()
                    .unwrap();
                let error_message_state = error_message_state.clone();
                let chat_id = chat_id.clone();

                if message_input_node.value().is_empty() {
                    return;
                }

                let response = send_message(
                    chat_id,
                    SendMessageRequest {
                        content: message_input_node.value(),
                        files: Vec::new(),
                    },
                )
                .await;

                if let Ok(response) = response {
                    if !response.ok {
                        error_message_state.set(Some("Не удалось отправить сообщение".to_string()));
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }

                message_input_node.set_value("");
            });
        })
    };

    html! {
        <>
            <Header/>
            if let Some(chat) = (*chat_state).clone() {
                <h1 class="fw-medium fs-1">
                    <img width=60px class="rounded-3" src={
                        if chat.icon.is_some() {
                            format!("/api/files/{}", chat.icon.unwrap_or("null.png".to_string()))
                        } else {
                            get_random_color_image_url(chat.name.clone(), 75, 75)
                        }
                    } />
                    <span class="p-3">
                        {chat.name}
                    </span>
                </h1>
            }

            <div class="row">
                <div class="col-lg-9 col-md-12 gy-3">
                    <div class="row gx-1">
                        <div class="col-lg-9 col-md-12 p-0">
                            <textarea ref={message_input_node} type="type" placeholder="Сообщение" class="form-control" />
                        </div>
                        <div class="col-1 d-none d-md-none d-lg-block"></div>
                        <div class="col-lg-2 col-md-12 p-0">
                            <button onclick={on_submit} class="col-12 m-0 btn btn-outline-success">{"Отправить"}</button>
                        </div>
                    </div>
                    <div class="col-12 overflow-y-scroll overflow-x-hidden align-items-center">
                        { for (*messages_state).iter().map(|message| html! {
                            <ChatMessage message={message.clone()}  />
                        }) }
                    </div>
                </div>

                <div class="col-lg-3 col-md-12 gy-3">
                    <div class="d-flex justify-content-between">
                        <h2 class="fs-2">
                            {"Участники"}
                        </h2>
                        <i class="bi bi-person-plus-fill grow-on-hover fs-3"></i>
                    </div>
                    <div class="d-flex grow-on-hover">
                        <img class="rounded-start-4 border object-fit-scale" height=60px src={
                            // if Option.is_some() {
                            //    format!("/api/files/{}", message.sender.icon.unwrap_or("null.png".to_string()))
                            // } else {
                                get_random_color_image_url("Aboba".to_string(), 60, 60)
                            // }
                        } alt="icon"/>
                        <div class="rounded-end-4 text-overflow-ellipsis d-flex border border-start-0 bg-white flex-grow-1 align-items-center">
                            <div class="p-3 fs-5 text-dark fw-normal">
                                {"Aboba"}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

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
