use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::{new_chat, upload_file, NewChatRequest},
    components::{ErrorMessage, Modal},
    routes::Route,
};

#[derive(PartialEq, Properties)]
pub struct NewChatModalButtonProps {
    pub children: Children,
    pub chat_name: String,
    pub redirect: bool,
}

#[function_component]
pub fn NewChatModalButton(props: &NewChatModalButtonProps) -> Html {
    let NewChatModalButtonProps {
        children,
        chat_name,
        redirect,
    } = props;

    let new_chat_dialog_visible = use_state(|| false);
    let icon_name_state = use_state(|| Option::<String>::None);
    let new_chat_input_node = use_node_ref();
    let new_chat_icon_node = use_node_ref();

    let navigator = use_navigator().unwrap();
    let error_message_state = use_state(|| Option::<String>::None);

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

    let on_file_upload = {
        let new_chat_icon_node = new_chat_icon_node.clone();
        let error_message_state = error_message_state.clone();
        let icon_name_state = icon_name_state.clone();

        Callback::from(move |_: InputEvent| {
            let icon_name_state = icon_name_state.clone();
            let new_chat_icon_node = new_chat_icon_node.clone();
            let error_message_state = error_message_state.clone();

            if let Some(files) = new_chat_icon_node.cast::<HtmlInputElement>() {
                let files = files.files().unwrap();

                if let Some(icon_file) = files.get(0) {
                    let error_message_state = error_message_state.clone();

                    spawn_local(async move {
                        let response = upload_file(icon_file).await;

                        if let Ok(response) = response {
                            if !response.ok {
                                error_message_state.set(Some("Что-то пошло не так".to_string()));
                            } else {
                                icon_name_state.set(response.data.unwrap().get(0).cloned());
                            }
                        } else {
                            error_message_state.set(Some("Сервер не отвечает".to_string()));
                        }
                    });
                }
            }
        })
    };

    let on_ok = {
        let error_message_state = error_message_state.clone();
        let new_chat_input_node = new_chat_input_node.clone();
        let icon_name_state = icon_name_state.clone();
        let redirect = redirect.clone();
        let navigator = navigator.clone();

        Callback::from(move |_: ()| {
            let error_message_state = error_message_state.clone();
            let new_chat_input_node = new_chat_input_node.clone();
            let icon_name_state = icon_name_state.clone();
            let navigator = navigator.clone();

            if new_chat_input_node.cast::<HtmlInputElement>().is_none() {
                return;
            }

            let input_element = new_chat_input_node.cast::<HtmlInputElement>().unwrap();
            let chat_name = input_element.value();

            if chat_name.is_empty() {
                error_message_state.set(Some("Чат не был создан".to_string()));
                return;
            }

            spawn_local(async move {
                let navigator = navigator.clone();
                let response = new_chat(NewChatRequest {
                    name: chat_name.clone(),
                    icon_file: (*icon_name_state).clone(),
                })
                .await;

                if let Ok(response) = response {
                    if !response.ok {
                        error_message_state.set(Some("Что-то пошло не так".to_string()));
                    } else {
                        if redirect.clone() {
                            navigator.push(&Route::Chat {
                                id: response.data.unwrap().id,
                            });
                        }
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }
            });
        })
    };

    html! {
        <>
        <span onclick={on_new_chat_click}>
            { for children.iter() }
        </span>

        <Modal modal_id={"new-chat-modal".to_string()} is_visible={*new_chat_dialog_visible} on_ok={on_ok} on_cancel={Callback::from(|_| {})} on_close={on_dialog_close}>
            <div class="modal-header">
                <h1 class="modal-title fs-5">{"Создать чат"}</h1>
            </div>
            <div class="modal-body">
                <div class="input-group p-1">
                    <span class="input-group-text">{"Имя чата"}</span>
                    <input value={chat_name.clone()} ref={new_chat_input_node} type="text" class="form-control" aria-label="chat_name" required=true />
                </div>
                <div class="input-group mb-3 p-1">
                    <label class="input-group-text">{"Загрузить иконку"}</label>
                    <input oninput={on_file_upload} type="file" ref={new_chat_icon_node} class="form-control" accept="image/png, image/jpeg, image/jpg" />
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
