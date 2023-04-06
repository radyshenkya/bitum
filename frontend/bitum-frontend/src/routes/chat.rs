use bitum_frontend::get_random_color_image_url;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::{
    api::{
        get_chat, get_chat_member, get_chat_members, get_messages, send_message, Chat, ChatMember,
        ChatMessage as ApiChatMessage, SendMessageRequest,
    },
    components::{
        AddChatMemberModalButton, ChatMemberButton, ChatMessage, ErrorMessage, Header,
        LoggedUserInfo,
    },
    constants::API_REFRESH_MILLIS,
};

#[derive(PartialEq, Properties)]
pub struct ChatRouteProps {
    pub chat_id: i32,
}

#[function_component]
pub fn ChatRoute(props: &ChatRouteProps) -> Html {
    let ChatRouteProps { chat_id } = props;
    let user = use_context::<LoggedUserInfo>().unwrap().user.unwrap();
    let error_message_state = use_state(|| Option::<String>::None);
    let self_chat_member_state = use_state(|| Option::<ChatMember>::None);
    let chat_state = use_state(|| Option::<Chat>::None);
    let message_input_node = use_node_ref();

    if self_chat_member_state.is_none() {
        let error_message_state = error_message_state.clone();
        let self_chat_member_state = self_chat_member_state.clone();
        let chat_id = chat_id.clone();
        let user = user.clone();

        spawn_local(async move {
            let error_message_state = error_message_state.clone();
            let self_chat_member_state = self_chat_member_state.clone();

            let response = get_chat_member(chat_id.clone(), user.id).await;

            if let Ok(response) = response {
                if response.ok {
                    self_chat_member_state.set(response.data);
                } else {
                    error_message_state
                        .set(Some("Не удалось получить информацию о себе".to_string()));
                }
            } else {
                error_message_state.set(Some("Сервер не отвечает".to_string()));
            }
        });
    }

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
                <div class="col-12 overflow-y-scroll overflow-x-hidden border rounded-5 align-items-center chat-messages-list">
                    <ChatMessagesList chat_id={*chat_id}/>
                </div>
            </div>

            <div class="col-lg-3 col-md-12 gy-3">
                <div class="d-flex justify-content-between">
                    <h2 class="fs-2">
                        {"Участники"}
                    </h2>
                    if let Some(chat_member) = (*self_chat_member_state).clone() {
                        if chat_member.permissions.can_add_members {
                            <AddChatMemberModalButton chat_id={*chat_id}>
                                <i class="bi bi-person-plus-fill fs-3"></i>
                            </AddChatMemberModalButton>
                        }
                    }
                </div>

                <ChatMembersList chat_id={*chat_id}/>
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

#[derive(PartialEq, Properties)]
struct ChatMessagesListProps {
    chat_id: i32,
}

#[function_component]
fn ChatMessagesList(props: &ChatMessagesListProps) -> Html {
    let ChatMessagesListProps { chat_id } = props;
    let messages_state = use_state(|| Vec::<ApiChatMessage>::new());
    let error_message_state = use_state(|| Option::<String>::None);

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

    html! {
        <>
        { for (*messages_state).iter().map(|message| html! {
            <ChatMessage message={message.clone()}  />
        }) }

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

#[derive(PartialEq, Properties)]
struct ChatMembersListProps {
    chat_id: i32,
}

#[function_component]
fn ChatMembersList(props: &ChatMembersListProps) -> Html {
    let ChatMembersListProps { chat_id } = props;
    let error_message_state = use_state(|| Option::<String>::None);
    let chat_members_state = use_state(|| Vec::<ChatMember>::new());

    {
        let error_message_state = error_message_state.clone();
        let chat_id = chat_id.clone();
        let chat_members_state = chat_members_state.clone();

        spawn_local(async move {
            TimeoutFuture::new(API_REFRESH_MILLIS).await;

            let response = get_chat_members(chat_id).await;

            if let Ok(response) = response {
                if response.ok {
                    chat_members_state.set(response.data.unwrap());
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
        { for (*chat_members_state).iter().map(|chat_member| html! {
            <ChatMemberButton member={chat_member.clone()}/>
        })}
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
