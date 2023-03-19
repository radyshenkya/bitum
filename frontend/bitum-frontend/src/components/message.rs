use bitum_frontend::{display_timestamp_date, parse_markdown_to_html};
use yew::prelude::*;

use crate::components::{LoggedUserInfo, RawHtml};

#[derive(PartialEq, Properties)]
pub struct ChatMessageProps {
    pub message: crate::api::ChatMessage,
}

#[function_component]
pub fn ChatMessage(props: &ChatMessageProps) -> Html {
    let ChatMessageProps { message } = props;

    let message = message.clone();
    let user = use_context::<LoggedUserInfo>().unwrap().user.unwrap();

    html! {
        <>
            <div class="row col-12 p-0">
                if message.sender.id == user.id {
                    <div class="col-lg-1 d-none d-lg-block d-md-none"></div>
                }

                <div class="message col-lg-11 col-md-12 g-col-6">
                    <div class="message-header d-flex">
                        <img src={format!("/api/files/{}", message.sender.icon.unwrap_or("null.png".to_string()))} class="border rounded-4 rounded-end-0 rounded-bottom-0" width=60px />
                        <div class="message-header-text justify-content-between rounded-4 rounded-start-0 rounded-bottom-0 p-2 d-flex align-items-center bg-body-secondary flex-grow-1">
                            <div class="fs-5 fw-normal">
                                { message.sender.username }
                            </div>
                            <div class="fs-5 p-2 text-body-secondary fw-light">
                                {display_timestamp_date(message.created_at as i64)}
                            </div>
                        </div>
                    </div>
                    <div class="p-3 message-body border border-top-0 rounded-4 rounded-top-0">
                        <RawHtml html={parse_markdown_to_html(message.content)} />
                    </div>
                </div>
            </div>
        </>
    }
}
