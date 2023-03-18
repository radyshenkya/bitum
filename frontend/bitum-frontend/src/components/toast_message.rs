use yew::prelude::*;

use crate::{components::Toast, TOAST_CONTAINER_ID};

#[derive(PartialEq, Properties)]
pub struct ToastMessageProps {
    pub children: Children,
    pub on_close: Callback<()>,
    pub class: Classes,
}

#[function_component]
pub fn ToastMessage(props: &ToastMessageProps) -> Html {
    let ToastMessageProps {
        children,
        on_close,
        class,
    } = props;

    let onclick = {
        let on_close = on_close.clone();

        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };

    html! {
        <Toast is_visible=true container_id={TOAST_CONTAINER_ID.to_string()}>
            <div class={classes!(class.clone(), vec!["toast", "show", "align-items-center", "border-0"])} role="alert" aria-live="assertive" data-bs-autohide="true" aria-atomic="true">
                <div class="d-flex">
                    <div class="toast-body">
                        {for children.iter()}
                    </div>
                    <button type="button" {onclick} class="btn-close btn-close-white me-2 m-auto" data-bs-dismiss="toast" aria-label="Close"></button>
                </div>
            </div>
        </Toast>
    }
}
