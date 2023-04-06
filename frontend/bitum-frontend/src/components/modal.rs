use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    pub children: Children,
    pub on_cancel: Callback<()>,
    pub on_ok: Callback<()>,
    pub on_close: Callback<()>,
    pub is_visible: bool,
    pub modal_id: String,
}

#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    let ModalProps {
        children,
        on_cancel,
        on_ok,
        on_close,
        is_visible,
        modal_id,
    } = props;

    let on_cancel_click = {
        let on_cancel = on_cancel.clone();
        let on_close = on_close.clone();

        Callback::from(move |_: MouseEvent| {
            on_cancel.emit(());
            on_close.emit(());
        })
    };

    let on_ok_click = {
        let on_ok = on_ok.clone();
        let on_close = on_close.clone();

        Callback::from(move |_: MouseEvent| {
            on_ok.emit(());
            on_close.emit(());
        })
    };

    html! {
        if *is_visible {
            <div class="modal" id={modal_id.clone()}>
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content shadow-lg">
                        { for children.iter() }
                        <div class="modal-footer">
                            <button type="button" onclick={on_cancel_click} class="btn btn-secondary" data-bs-dismiss="modal">{"Отмена"}</button>
                            <button type="button" onclick={on_ok_click} class="btn btn-primary" data-bs-dismiss="modal">{"ОК"}</button>
                        </div>
                    </div>
                </div>
            </div>
        } else {
            <></>
        }
    }
}
