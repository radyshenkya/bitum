use yew::prelude::*;

use crate::components::ToastMessage;

#[derive(PartialEq, Properties)]
pub struct TextMessageProps {
    pub value: String,
    pub on_close: Callback<()>,
    pub toast_classes: Classes,
}

#[derive(Clone, Debug)]
pub struct TextMessageState {
    pub value: String,
    pub classes: Classes,
}

#[function_component]
pub fn TextMessage(props: &TextMessageProps) -> Html {
    let TextMessageProps {
        value,
        on_close,
        toast_classes,
    } = props;

    html! {
        <ToastMessage on_close={on_close.clone()} class={classes!(toast_classes.clone())} >
            {value.clone()}
        </ToastMessage>
    }
}
