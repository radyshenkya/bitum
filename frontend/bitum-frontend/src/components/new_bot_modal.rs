use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    api::new_bot,
    components::{ErrorMessage, Modal},
};

#[derive(PartialEq, Properties)]
pub struct NewBotModalButtonProps {
    pub children: Children,
    pub username: String,
}

#[function_component]
pub fn NewBotModalButton(props: &NewBotModalButtonProps) -> Html {
    let NewBotModalButtonProps { children, username } = props;

    let error_message_state = use_state(|| Option::<String>::None);
    let dialog_visible = use_state(|| false);
    let bot_username_node_ref = use_node_ref();

    let on_close = {
        let dialog_visible = dialog_visible.clone();

        Callback::from(move |_: ()| {
            dialog_visible.set(false);
        })
    };

    let on_ok = {
        let on_close = on_close.clone();
        let bot_username_node_ref = bot_username_node_ref.clone();
        let error_message_state = error_message_state.clone();

        Callback::from(move |_: ()| {
            let error_message_state = error_message_state.clone();
            let bot_username = bot_username_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            spawn_local(async move {
                let bot_username = bot_username.clone();

                let response = new_bot(bot_username).await;

                if let Ok(response) = response {
                    if !response.ok {
                        error_message_state.set(Some("Что-то пошло не так".to_string()));
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }
            });

            on_close.emit(());
        })
    };

    let on_open = {
        let dialog_visible = dialog_visible.clone();

        Callback::from(move |_: MouseEvent| {
            dialog_visible.set(true);
        })
    };

    html! {
        <>
        <span onclick={on_open}>
            { for children.iter() }
        </span>

        <Modal modal_id={"new-bot-modal".to_string()} is_visible={*dialog_visible} on_ok={on_ok} on_cancel={Callback::from(|_| {})} on_close={on_close}>
            <div class="modal-header">
                <h1 class="modal-title fs-5">{"Создать бота"}</h1>
            </div>
            <div class="modal-body">
                <div class="input-group p-1">
                    <span class="input-group-text">{"Имя бота"}</span>
                    <input value={username.clone()} ref={bot_username_node_ref} type="text" class="form-control" aria-label="bot_username" required=true />
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
