use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BsToastProps {
    pub children: Children,
}

#[function_component]
pub fn BsToast(props: &BsToastProps) -> Html {
    let BsToastProps { children } = props;
    html! {
        <div class="toast-container position-fixed bottom-0 p-3 end-0">
            { for children.iter() }
        </div>
    }
}
