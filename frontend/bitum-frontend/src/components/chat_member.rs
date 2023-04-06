use bitum_frontend::get_random_color_image_url;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::{delete_chat_member, ChatMember},
    components::{ErrorMessage, LoggedUserInfo},
};

#[derive(PartialEq, Properties)]
pub struct ChatMemberButtonProps {
    pub member: ChatMember,
}

#[function_component]
pub fn ChatMemberButton(props: &ChatMemberButtonProps) -> Html {
    let ChatMemberButtonProps { member } = props;
    let user = use_context::<LoggedUserInfo>().unwrap().user.unwrap();
    let error_message_state = use_state(|| Option::<String>::None);

    let on_delete_click = {
        let error_message_state = error_message_state.clone();
        let member = member.clone();

        Callback::from(move |_: MouseEvent| {
            let error_message_state = error_message_state.clone();

            spawn_local(async move {
                let response = delete_chat_member(member.chat.id, member.user.id).await;

                if let Ok(response) = response {
                    if !response.ok {
                        error_message_state.set(Some("Не удалось удалить бота".to_string()));
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }
            });
        })
    };

    html! {
        <>
        <div class="d-flex chat-member-button">
            <img class="rounded-start-4 border object-fit-scale" height=70px src={
                if member.user.icon.is_some() {
                    format!("/api/files/{}", member.user.icon.clone().unwrap_or("null.png".to_string()))
                } else {
                    get_random_color_image_url(member.user.username.clone(), 70, 70)
                }
            } alt="icon"/>
            <div class="rounded-end-4 text-overflow-ellipsis justify-content-between d-flex border border-start-0 bg-white flex-grow-1 align-items-center">
                <div class="p-3 fs-5 text-dark fw-normal">
                    if member.user.is_bot {
                        <span class="badge rounded-pill text-bg-dark">{"бот"}</span>
                        {" "}
                    }
                    { member.user.username.clone() }
                </div>
                if member.chat.owner.id == user.id {
                    <i onclick={on_delete_click} role="button" class="bi bi-trash2 fs-3 p-3 danger-hover"></i>
                }
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
