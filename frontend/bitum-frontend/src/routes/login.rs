use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};

use crate::{
    api::{get_user_token, GetTokenRequest},
    components::{ErrorMessage, Footer, Header},
    routes::Route,
};

#[derive(PartialEq, Properties)]
pub struct LoginRouteProps {}

#[function_component]
pub fn LoginRoute(props: &LoginRouteProps) -> Html {
    let LoginRouteProps {} = props;

    let error_message_state = use_state(|| Option::<String>::None);
    let username_input_node = use_node_ref();
    let password_input_node = use_node_ref();
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let username_input_node = username_input_node.clone();
        let password_input_node = password_input_node.clone();
        let error_message_state = error_message_state.clone();

        Callback::from(move |submit_event: SubmitEvent| {
            let navigator = navigator.clone();
            let error_message_state = error_message_state.clone();
            submit_event.prevent_default();

            if [username_input_node.clone(), password_input_node.clone()]
                .iter()
                .any(|input| input.cast::<HtmlInputElement>().is_none())
            {
                return;
            }

            let username = username_input_node
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            let password = password_input_node
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            spawn_local(async move {
                let response = get_user_token(GetTokenRequest {
                    username: username.clone(),
                    password: password.clone(),
                })
                .await;

                let mut error = Option::<String>::None;

                if let Ok(resp) = response {
                    if !resp.ok {
                        match resp.error.unwrap().code {
                            404 => error = Some(String::from("Ошибка! Такого пользователя нет.")),
                            _ => error = Some(String::from("Неизвестная ошибка!")),
                        }
                    } else {
                        navigator.push(&Route::Chats);
                    }
                } else {
                    error = Some(String::from("Неизвестная ошибка в работе сервера."))
                }

                error_message_state.set(error);
            });
        })
    };

    html! {
        <>
        <Header />
        <div class="text-center row justify-content-md-center">
            <div class="col-md-4 col-sm-12">
                <h1 class="fw-light fs-2 p-4">{"Вход"}</h1>
                <form class="text-start gy-3 row input-group-lg" {onsubmit}>
                    <div class="col-12 input-group-lg">
                        <label class="fw-medium fs-5"  >{"Имя пользователя"}</label>
                        <input ref={username_input_node} type="text" placeholder="username" class="form-control" id="username-input" required=true />
                    </div>

                    <div class="col-12 input-group-lg">
                        <label class="fw-medium fs-5" for="password-input">{"Пароль"}</label>
                        <input ref={password_input_node} type="password" placeholder="password" class="form-control" id="password-input" required=true />
                    </div>

                    <div class="col-12 d-flex p-2 justify-content-between">
                        <button class="btn btn-success btn-lg" type="submit" id="submit-button">{"Войти"}</button>
                        <Link<Route> classes="btn btn-light btn-lg" to={Route::Register}>{"Регистрация"}</Link<Route>>
                    </div>
                </form>
            </div>
        </div>
        <Footer />
        if let Some(err) = (*error_message_state).clone() {
            <ErrorMessage value={err}/>
        }
        </>
    }
}
