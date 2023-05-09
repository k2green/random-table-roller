use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditableHeaderProps {
    pub text: String,
    pub callback: Callback<String>
}

#[function_component(EditableHeader)]
pub fn editable_header(props: &EditableHeaderProps) -> Html {
    let EditableHeaderProps { text, callback } = props.clone();
    let is_editing = use_state_eq(|| false);

    let set_editing = {
        let is_editing = is_editing.clone();
        Callback::from(move |_: MouseEvent| {
            is_editing.set(true);
        })
    };

    let set_not_editing = {
        let is_editing = is_editing.clone();
        Callback::from(move |_: FocusEvent| {
            is_editing.set(false);
        })
    };

    let update_text = {
        let is_editing = is_editing.clone();
        let callback = callback.clone();

        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let text = target.value();
            let trimmed_text = text.trim();

            is_editing.set(false);

            if !trimmed_text.is_empty() {
                callback.emit(trimmed_text.to_string());
            }
        })
    };

    html! {
        if *is_editing {
            <input class="stretch-width table-name" value={text} onchange={update_text} onfocusout={set_not_editing} />
        } else {
            <h2 class="heading table-name pointer" ondblclick={set_editing}>{text}</h2>
        }
    }
}