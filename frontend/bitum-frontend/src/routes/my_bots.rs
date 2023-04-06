use bitum_frontend::get_random_color_image_url;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::{delete_bot, get_bot_token, get_bots, User},
    components::{ErrorMessage, Header, LoggedUserInfo, Modal, NewBotModalButton},
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
                        <BotCard bot={bot.clone()} />
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

#[derive(PartialEq, Properties)]
pub struct BotCardProps {
    pub bot: User,
}

#[function_component]
pub fn BotCard(props: &BotCardProps) -> Html {
    let BotCardProps { bot } = props;

    let error_message_state = use_state(|| Option::<String>::None);
    let token_modal_visible = use_state(|| false);
    let token = use_state(|| String::new());

    let on_close = {
        let token_modal_visible = token_modal_visible.clone();

        Callback::from(move |_: ()| {
            token_modal_visible.set(false);
        })
    };

    let on_token_click = {
        let token_modal_visible = token_modal_visible.clone();
        let token = token.clone();
        let bot = bot.clone();
        let error_message_state = error_message_state.clone();

        Callback::from(move |_: MouseEvent| {
            let token_modal_visible = token_modal_visible.clone();
            let error_message_state = error_message_state.clone();
            let token = token.clone();

            spawn_local(async move {
                let response = get_bot_token(bot.id.clone()).await;

                if let Ok(response) = response {
                    if response.ok {
                        token.set(response.data.unwrap().token);
                        token_modal_visible.set(true);
                    } else {
                        error_message_state.set(Some("Не удалось получить токен бота".to_string()));
                    }
                } else {
                    error_message_state.set(Some("Сервер не отвечает".to_string()));
                }
            });
        })
    };

    let on_delete_click = {
        let bot = bot.clone();
        let error_message_state = error_message_state.clone();

        Callback::from(move |_: MouseEvent| {
            let error_message_state = error_message_state.clone();

            spawn_local(async move {
                let response = delete_bot(bot.id.clone()).await;

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
        <div class="col-lg-4 col-md-12 p-2">
            <div class="d-flex">
                <img class="rounded-start-2 border object-fit-scale" height=75px src={
                    if bot.icon.is_some() {
                        format!("/api/files/{}", (*bot).clone().icon.unwrap_or("null.png".to_string()))
                    } else {
                        get_random_color_image_url(bot.username.clone(), 75, 75)
                    }
            } alt="Bot icon"/>
                <div class="rounded-end-2 text-overflow-ellipsis justify-content-between d-flex border border-start-0 bg-white flex-grow-1 align-items-center">
                    <div class="p-3 fs-4 text-dark fw-normal">
                        {bot.username.clone()}
                    </div>
                    <div>
                        <i onclick={on_token_click} role="button" class="bi bi-key fs-3 p-1 info-hover"></i>
                        <i onclick={on_delete_click} role="button" class="bi bi-trash2 fs-3 p-3 danger-hover"></i>
                    </div>
                </div>
            </div>
        </div>
        <Modal modal_id={"bot-token-modal".to_string()} is_visible={*token_modal_visible} on_ok={Callback::from(|_| {})} on_cancel={Callback::from(|_| {})} on_close={on_close}>
            <div class="modal-header">
                <h1 class="modal-title fs-5">{"Токен "} {bot.username.clone()}</h1>
            </div>
            <div class="modal-body">
                <div class="input-group flex-nowrap">
                    <span class="input-group-text" id="addon-wrapping">
                        <i class="bi bi-key fs-4"></i>
                    </span>
                    <input class="form-control" type="text" value={(*token).clone()} readonly=true />
                </div>
            </div>
        </Modal>
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
