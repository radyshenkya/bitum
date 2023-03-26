use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    api::{add_chat_member, search_bots, search_users, User},
    components::{ErrorMessage, Modal},
};

#[derive(PartialEq, Properties)]
pub struct AddChatMemberModalButtonProps {
    pub chat_id: i32,
    pub children: Children,
}

#[function_component]
pub fn AddChatMemberModalButton(props: &AddChatMemberModalButtonProps) -> Html {
    let AddChatMemberModalButtonProps { chat_id, children } = props;

    let modal_visible = use_state(|| false);
    let error_message_state = use_state(|| Option::<String>::None);
    let found_users_state = use_state(|| Vec::<User>::new());
    let input_node_ref = use_node_ref();

    let on_button_click = {
        let modal_visible = modal_visible.clone();

        Callback::from(move |_: MouseEvent| {
            modal_visible.set(true);
        })
    };

    let on_modal_close = {
        let modal_visible = modal_visible.clone();

        Callback::from(move |_: ()| {
            modal_visible.set(false);
        })
    };

    let member_button_callback = {
        let chat_id = chat_id.clone();
        let error_message_state = error_message_state.clone();
        let on_modal_close = on_modal_close.clone();

        Callback::from(move |user_id: i32| {
            let chat_id = chat_id.clone();
            let error_message_state = error_message_state.clone();
            let on_modal_close = on_modal_close.clone();

            spawn_local(async move {
                let response = add_chat_member(chat_id, user_id).await;

                if let Ok(response) = response {
                    if response.ok {
                        on_modal_close.emit(());
                    } else {
                        match response.error.unwrap().code {
                            409 => error_message_state.set(Some(
                                "Этот пользователь уже состоит в этом чате".to_string(),
                            )),
                            _ => error_message_state
                                .set(Some("Не удалось добавить пользователя в чат".to_string())),
                        }
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }
            });
        })
    };

    let on_input_change = {
        let input_node_ref = input_node_ref.clone();
        let error_message_state = error_message_state.clone();
        let found_users_state = found_users_state.clone();

        Callback::from(move |_: InputEvent| {
            let username = input_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let error_message_state = error_message_state.clone();
            let found_users_state = found_users_state.clone();

            if username.is_empty() {
                return;
            }

            spawn_local(async move {
                let response = search_users(username.clone(), 40, 0).await;

                let mut new_user_list = Vec::new();

                if let Ok(response) = response {
                    if response.ok {
                        new_user_list = response.data.unwrap();
                    } else {
                        error_message_state.set(Some(
                            "Не удалось получить информацию о пользователях".to_string(),
                        ));
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }

                let response = search_bots(username, 40, 0).await;

                if let Ok(response) = response {
                    if response.ok {
                        new_user_list.extend(response.data.unwrap());
                    } else {
                        error_message_state.set(Some(
                            "Не удалось получить информацию о пользователях".to_string(),
                        ));
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }

                found_users_state.set(new_user_list);
            });
        })
    };

    html! {
        <>
        <span onclick={on_button_click}>
            { for children.iter() }
        </span>

        <Modal modal_id={"add-chat-member-modal"} is_visible={*modal_visible} on_close={on_modal_close} on_cancel={Callback::from(move |_| {})} on_ok={ Callback::from(move |_: ()| {}) } >
            <div class="modal-header">
                <h1 class="modal-title fs-5">{"Добавить пользователя в чат"}</h1>
            </div>
            <div class="modal-body">
                <div class="input-group p-1">
                    <span class="input-group-text">{"Имя пользователя"}</span>
                    <input oninput={on_input_change} ref={input_node_ref} type="text" class="form-control" aria-label="username" required=true />
                </div>
                <div class="d-block add-member-list-modal">
                    { for (*found_users_state).iter().map(|user| html! {
                        <div
                            onclick={
                                let user = user.clone();
                                let member_button_callback = member_button_callback.clone();
                                Callback::from(move |_| {
                                    member_button_callback.clone().emit(user.id);
                                })
                            }
                            role="button"
                            class="d-flex chat-member-button grow-on-hover"
                            >
                                <div class="rounded-2 text-overflow-ellipsis d-flex border bg-white flex-grow-1 align-items-center">
                                <div class="p-3 fs-5 text-dark fw-normal">
                                    if user.is_bot {
                                        <span class="badge rounded-pill text-bg-dark">{"бот"}</span>
                                        {" "}
                                    }
                                    { user.username.clone() }
                                </div>
                            </div>
                        </div>
                    })}
                </div>
            </div>
        </Modal>
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
