use log::info;
use yew::prelude::*;

use crate::components::BsToast;

#[derive(PartialEq, Properties)]
pub struct ErrorMessageProps {
    pub value: String,
}

#[function_component]
pub fn ErrorMessage(props: &ErrorMessageProps) -> Html {
    let ErrorMessageProps { value } = props;

    html! {
        <BsToast>
            <div class="toast show align-items-center bg-danger text-bg-primary border-0" role="alert" aria-live="assertive" data-bs-autohide="true" aria-atomic="true">
                <div class="d-flex">
                    <div class="toast-body">
                        {value.clone()}
                    </div>
                    <button type="button" class="btn-close btn-close-white me-2 m-auto" data-bs-dismiss="toast" aria-label="Close"></button>
                </div>
            </div>
        </BsToast>
    }
}
