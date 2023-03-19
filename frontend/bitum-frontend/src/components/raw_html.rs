// GOT FROM https://github.com/yewstack/yew/issues/1281#issuecomment-1032906972
use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct RawHtmlProps {
    pub html: String,
}

#[function_component]
pub fn RawHtml(props: &RawHtmlProps) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}
