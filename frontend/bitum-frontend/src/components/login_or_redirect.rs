use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::{get_current_user_info, User},
    routes::Route,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LoggedUserInfo {
    user: Option<User>,
}

#[derive(PartialEq, Properties)]
pub struct LoginOrRedirectProps {
    pub children: Children,
}

#[function_component]
pub fn LoginOrRedirect(props: &LoginOrRedirectProps) -> Html {
    let LoginOrRedirectProps { children } = props;

    let logged_user_info = use_state(|| LoggedUserInfo { user: None });
    let navigator = use_navigator().unwrap();

    {
        let logged_user_info = logged_user_info.clone();

        spawn_local(async move {
            if logged_user_info.user.is_some() {
                return;
            }

            let response = get_current_user_info().await;

            if let Ok(resp) = response {
                if resp.ok {
                    logged_user_info.set(LoggedUserInfo { user: resp.data });
                } else {
                    navigator.push(&Route::Login);
                }
            } else {
                navigator.push(&Route::Login);
            }
        });
    }

    html! {
        <ContextProvider<LoggedUserInfo> context={(*logged_user_info).clone()}>
            { for children.iter() }
        </ContextProvider<LoggedUserInfo>>
    }
}
