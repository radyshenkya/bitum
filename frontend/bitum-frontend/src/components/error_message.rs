use yew::prelude::*;

use crate::{
    components::{ToastMessage},
};

#[derive(PartialEq, Properties)]
pub struct ErrorMessageProps {
    pub value: String,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn ErrorMessage(props: &ErrorMessageProps) -> Html {
    let ErrorMessageProps { value, on_close } = props;

    html! {
        <ToastMessage on_close={on_close.clone()} class={classes!("bg-danger", "text-bg-primary")} >
            {value.clone()}
        </ToastMessage>
    }
}
