use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{routes::Route, components::Footer};

#[derive(PartialEq, Properties)]
pub struct IndexRouteProps {}

#[function_component]
pub fn IndexRoute(props: &IndexRouteProps) -> Html {
    let IndexRouteProps {} = props;
    html! {
        <>
        <div class="d-block text-center">
            <div class="index-logo">
                <div class="row">
                    <div class="col">
                        <img src="static/img/bitum-icon.svg" width=130px />
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <h1 class="fw-light fs-1">{"Bitum"}</h1>
                    </div>
                </div>
            </div>
            <div class="index-text">
                <div class="row">
                    <div class="col">
                        <h2 class="fs-2 fw-normal text-dark-emphasis">
                            {"Мессенджер с поддержкой ботов"}
                        </h2>
                        <h3 class="fw-light fs-3">
                            {"Учебный проект в Яндекс Лицей"}
                        </h3>
                    </div>
                </div>
            </div>
            <div class="index-links">
                <div class="row">
                    <div class="col">
                            <Link<Route> classes="btn btn-success btn-lg" to={Route::Register}>{"Зарегистрироваться"}</Link<Route>>
                            <Link<Route> classes="btn btn-light btn-lg" to={Route::Login}>{"Войти"}</Link<Route>>
                    </div>
                </div>
            </div>
        </div>
        <Footer />
        </>
    }
}
