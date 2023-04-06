use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ToastContainerProps {
    pub id: String,
}

#[function_component]
pub fn ToastContainer(props: &ToastContainerProps) -> Html {
    let ToastContainerProps { id } = props;
    html! {
        <div id={id.clone()} class="toast-container position-fixed bottom-0 end-0 p-3"></div>
    }
}
