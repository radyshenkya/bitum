use log::info;
use yew::prelude::*;

use crate::components::{Footer, Header, Modal};

#[derive(PartialEq, Properties)]
pub struct ChatsRouteProps {}

#[function_component]
pub fn ChatsRoute(props: &ChatsRouteProps) -> Html {
    let ChatsRouteProps {} = props;

    let new_chat_dialog_visible = use_state(|| false);

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
        Callback::from(move |_: ()| {
            info!("OK CLICKED");
        })
    };

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
                        <input type="text" class="form-control" aria-label="chat_name" required=true />
                    </div>
                </div>
            </Modal>
            <h1 class="fw-medium fs-1">
                {"Чаты "}
                <button onclick={on_new_chat_click} class="btn btn-light btn-lg">
                    <i class="bi bi-plus-square-fill fs-4"></i>
                    // <!-- {" Новый чат"} -->
                </button>
            </h1>
            <Footer/>
        </>
    }
}
