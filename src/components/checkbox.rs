use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CheckboxProps {
    pub checked: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_change: Callback<bool>
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps { checked, class, on_change } = props.clone();

    let checkbox_toggled = {
        let on_change = on_change.clone();

        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            on_change.emit(target.checked());
        })
    };

    html! {
        <div class={class}>
            <label class="switch">
                <input type="checkbox" checked={checked} onchange={checkbox_toggled} />
                <span class="slider" />
            </label>
        </div>
    }
}