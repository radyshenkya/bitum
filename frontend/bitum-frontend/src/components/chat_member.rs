use bitum_frontend::get_random_color_image_url;
use yew::prelude::*;

use crate::api::ChatMember;

#[derive(PartialEq, Properties)]
pub struct ChatMemberButtonProps {
    pub member: ChatMember,
}

#[function_component]
pub fn ChatMemberButton(props: &ChatMemberButtonProps) -> Html {
    let ChatMemberButtonProps { member } = props;

    html! {
        <>
            <div role="button" class="d-flex chat-member-button grow-on-hover">
                <img class="rounded-start-4 border object-fit-scale" height=70px src={
                    if member.user.icon.is_some() {
                       format!("/api/files/{}", member.user.icon.clone().unwrap_or("null.png".to_string()))
                    } else {
                        get_random_color_image_url(member.user.username.clone(), 70, 70)
                    }
                } alt="icon"/>
                <div class="rounded-end-4 text-overflow-ellipsis d-flex border border-start-0 bg-white flex-grow-1 align-items-center">
                    <div class="p-3 fs-5 text-dark fw-normal">
                        if member.user.is_bot {
                            <span class="badge rounded-pill text-bg-dark">{"бот"}</span>
                            {" "}
                        }
                        { member.user.username.clone() }
                    </div>
                </div>
            </div>
        </>
    }
}
