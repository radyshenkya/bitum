use wasm_bindgen_futures::spawn_local;
use yew::*;
use yew_router::prelude::use_navigator;

use crate::{api::get_current_user_info, routes::Route, LoggedUserInfo};

#[function_component]
pub fn LoginOrRedirect() -> Html {
    let logged_user_reducer = use_reducer(|| LoggedUserInfo { user: None });
    let navigator = use_navigator().unwrap();

    spawn_local(async move {
        let response = get_current_user_info().await;

        log::info!("{:?}", response);

        if let Ok(resp) = response {
            if resp.ok {
                logged_user_reducer.dispatch(resp.data);
            } else {
                navigator.push(&Route::Login);
            }
        } else {
            navigator.push(&Route::Login);
        }
    });

    html! {
        <></>
    }
}
