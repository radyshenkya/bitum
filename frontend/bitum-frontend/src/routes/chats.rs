use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    api::{get_chats, new_chat, Chat, NewChatRequest},
    components::{ErrorMessage, Footer, Header, Modal},
    constants::API_REFRESH_MILLIS,
    routes::Route,
};

#[derive(PartialEq, Properties)]
pub struct ChatsRouteProps {}

#[function_component]
pub fn ChatsRoute(props: &ChatsRouteProps) -> Html {
    let ChatsRouteProps {} = props;

    let error_message_state = use_state(|| Option::<String>::None);

    let new_chat_dialog_visible = use_state(|| false);
    let new_chat_input_node = use_node_ref();

    let chats_state = use_state(|| Vec::new());

    let on_new_chat_click = {
        let new_chat_dialog_visible = new_chat_dialog_visible.clone();

        Callback::from(move |_: MouseEvent| {
            new_chat_dialog_visible.set(true);
        })
    };

    let on_dialog_close = {
        let new_chat_dialog_visible = new_chat_dialog_visible.clone();

        Callback::from(move |_: ()| {
            new_chat_dialog_visible.set(false);
        })
    };

    let on_ok = {
        let error_message_state = error_message_state.clone();
        let new_chat_input_node = new_chat_input_node.clone();

        Callback::from(move |_: ()| {
            let error_message_state = error_message_state.clone();
            let new_chat_input_node = new_chat_input_node.clone();

            if let Some(input_element) = new_chat_input_node.cast::<HtmlInputElement>() {
                let chat_name = input_element.value();

                if chat_name.is_empty() {
                    error_message_state.set(Some("Чат не был создан".to_string()));
                    return;
                }

                spawn_local(async move {
                    let response = new_chat(NewChatRequest {
                        name: chat_name.clone(),
                        icon_file: None,
                    })
                    .await;

                    if let Ok(response) = response {
                        if !response.ok {
                            error_message_state.set(Some("Что-то пошло не так".to_string()));
                        }
                    } else {
                        error_message_state.set(Some("Сервер не отвечает".to_string()));
                    }
                });
            }
        })
    };

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
            <Modal modal_id={"new-chat-modal".to_string()} is_visible={*new_chat_dialog_visible} on_ok={on_ok} on_cancel={Callback::from(|_| {})} on_close={on_dialog_close}>
                <div class="modal-header">
                    <h1 class="modal-title fs-5">{"Создать чат"}</h1>
                </div>
                <div class="modal-body">
                    <div class="input-group">
                        <span class="input-group-text">{"Имя чата"}</span>
                        <input ref={new_chat_input_node} type="text" class="form-control" aria-label="chat_name" required=true />
                    </div>
                </div>
            </Modal>
            <h1 class="fw-medium fs-1">
                {"Чаты "}
                <button onclick={on_new_chat_click} class="btn btn-light btn-lg">
                    <i class="bi bi-plus-square-fill fs-4"></i>
                </button>
            </h1>
            <div class="row">
                {
                    for (*chats_state).iter().map(|chat| {
                        html! {
                            <div class="col-4 p-2">
                                <Link<Route> classes="text-decoration-none d-flex grow-on-hover" to={Route::Chat {id: chat.id.clone()}}>
                                    <img class="rounded-start-2" width=60px src={format!("/api/files/{}", chat.icon.clone().unwrap_or("null.png".to_string()))} alt="Chat icon"/>
                                    <div class="rounded-end-2 d-flex border border-left-0 bg-white flex-grow-1 align-items-center">
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
