use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FooterProps {}

#[function_component]
pub fn Footer(props: &FooterProps) -> Html {
    let FooterProps {} = props;
    html! {
        <footer class="fixed-bottom footer d-flex text-center justify-content-center align-items-center">
            <div class="p-3 fs-4">
                <a href="https://github.com/radyshenkya" target="blank" class="text-decoration-none text-light-emphasis">{"мой github"}</a>
            </div>
            <div class="p-3 fs-3">
                {"//"}
            </div>
            <div class="p-3 fs-4">
                <a href="https://github.com/radyshenkya/bitum" target="blank" class="text-decoration-none text-light-emphasis">{"github проекта"}</a>
            </div>
            <div class="p-3 fs-3">
                {"//"}
            </div>
            <div class="p-3 fs-4">
                <a href="https://t.me/c4pslokk" target="blank" class="text-decoration-none text-light-emphasis">{"телеграм"}</a>
            </div>
        </footer>
    }
}
