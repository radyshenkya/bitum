use yew::prelude::*;

use crate::components::LoggedUserInfo;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let HeaderProps {} = props;
    let logged_user_info = use_context::<LoggedUserInfo>();

    html! {
        <header class="header row">
            <div class="icon d-flex col-3 align-items-center">
                <a href="/" class="d-block text-decoration-none text-reset">
                    <img src="static_files/img/bitum-icon.svg" alt="icon" width=70px />
                </a>
                <div class="fw-light fs-1 p-4">{"Bitum"}</div>
            </div>
            {format!("{:?}", logged_user_info)}
        </header>
    }
}
