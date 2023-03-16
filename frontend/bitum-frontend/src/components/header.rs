use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{components::LoggedUserInfo, routes::Route};

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

// TODO: Добавить линки на Чаты Боты Мои боты
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
            if let Some(logged_user_info) = logged_user_info {
                if let Some(user) = logged_user_info.user {
                    <nav class="col-md-6 col-sm-12 d-flex text-center justify-content-center align-items-center">
                        <div class="p-3 fs-5">
                            <Link<Route> classes="text-decoration-none text-light-emphasis" to={Route::Chats}>{"Чаты"}</Link<Route>>
                        </div>
                        <div class="p-3 fs-4">
                            {"//"}
                        </div>
                        <div class="p-3 fs-5">
                            <a href="#" target="blank" class="text-decoration-none text-light-emphasis">{"Боты"}</a>
                        </div>
                        <div class="p-3 fs-4">
                            {"//"}
                        </div>
                        <div class="p-3 fs-5">
                            <a href="#" target="blank" class="text-decoration-none text-light-emphasis">{"Мои боты"}</a>
                        </div>
                    </nav>

                    <div class="col-md-3 col-sm-12 d-flex text-center justify-content-center align-items-center">
                        <div class="fs-3 fw-medium">
                            {user.username}
                        </div>
                    </div>
                }
            }
        </header>
    }
}
