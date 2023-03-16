use yew::prelude::*;

use crate::components::Header;

#[derive(PartialEq, Properties)]
pub struct ChatsRouteProps {}

#[function_component]
pub fn ChatsRoute(props: &ChatsRouteProps) -> Html {
    let ChatsRouteProps {} = props;

    html! {
        <>
            <Header/>
        </>
    }
}
