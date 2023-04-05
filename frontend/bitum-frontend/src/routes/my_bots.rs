use bitum_frontend::get_random_color_image_url;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::{get_bots, User},
    components::{ErrorMessage, Header, LoggedUserInfo, NewBotModalButton},
    constants::API_REFRESH_MILLIS,
};

#[derive(PartialEq, Properties)]
pub struct MyBotsRouteProps {}

#[function_component]
pub fn MyBotsRoute(props: &MyBotsRouteProps) -> Html {
    let MyBotsRouteProps {} = props;
    let logged_user = use_context::<LoggedUserInfo>();
    let error_message_state = use_state(|| Option::<String>::None);
    let bots_state = use_state(|| Vec::<User>::new());

    {
        let error_message_state = error_message_state.clone();
        let bots_state = bots_state.clone();

        spawn_local(async move {
            TimeoutFuture::new(API_REFRESH_MILLIS).await;
            let response = get_bots().await;

            if let Ok(response) = response {
                if response.ok {
                    bots_state.set(response.data.unwrap());
                } else {
                    error_message_state.set(Some("Не удалось получить список чатов".to_string()));
                }
            } else {
                error_message_state.set(Some("Сервер не отвечает".to_string()));
            }
        });
    }

    html! {
        <>
        <Header />

        <h1 class="fw-medium fs-1">
            {"Мои боты "}
            <NewBotModalButton username={"".to_string()}>
                <i class="bi bi-plus-square-fill fs-1 p-3"></i>
            </NewBotModalButton>
        </h1>

        if let Some(user) = logged_user {
            if let Some(_) = user.user {
                <div class="row">
                { for (*bots_state).iter().map(|bot| {
                    html! {
                        <div class="col-lg-4 col-md-12 p-2">
                            <div class="d-flex grow-on-hover">
                                <img class="rounded-start-2 border object-fit-scale" height=75px src={
                                    if bot.icon.is_some() {
                                        format!("/api/files/{}", (*bot).clone().icon.unwrap_or("null.png".to_string()))
                                    } else {
                                        get_random_color_image_url(bot.username.clone(), 75, 75)
                                    }
                            } alt="Chat icon"/>
                                <div class="rounded-end-2 text-overflow-ellipsis d-flex border border-start-0 bg-white flex-grow-1 align-items-center">
                                    <div class="p-3 fs-4 text-dark fw-normal">
                                        {bot.username.clone()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })}
                </div>
            }
        }

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
