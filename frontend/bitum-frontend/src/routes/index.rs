use yew::prelude::*;

use crate::components::LoginOrRedirect::LoginOrRedirect;

#[derive(PartialEq, Properties)]
pub struct IndexRouteProps {}

#[function_component]
pub fn IndexRoute(props: &IndexRouteProps) -> Html {
    let IndexRouteProps {} = props;
    html! {
        <div>
            <LoginOrRedirect/>
        </div>
    }
}
