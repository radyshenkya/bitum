use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ToastProps {
    pub children: Children,
    pub is_visible: bool,
    pub container_id: String,
}

#[function_component]
pub fn Toast(props: &ToastProps) -> Html {
    let ToastProps {
        children,
        is_visible,
        container_id,
    } = props;

    let toast_host = gloo::utils::document()
        .get_element_by_id(container_id)
        .expect(&format!("Expected to find #{} element", container_id));

    create_portal(
        html! {
            if *is_visible { <>{for children.iter()}</> }
            else {<></>}
        },
        toast_host.into(),
    )
}
