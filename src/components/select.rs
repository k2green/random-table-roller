use std::{fmt::Display, sync::Arc};

use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectProps<T: Clone + PartialEq + Display + 'static> {
    pub items: Arc<Vec<T>>,
    #[prop_or_default]
    pub on_change: Callback<T>
}

#[function_component(Select)]
pub fn select<T: Clone + PartialEq + Display + 'static>(props: &SelectProps<T>) -> Html {
    let SelectProps { items, on_change } = props.clone();
    let selected_index = use_state_eq(|| 0_usize);
    let is_focused = use_state_eq(|| false);
    let selected_item = &items[*selected_index];
    let icon_id = if *is_focused { IconId::BootstrapCaretUpFill } else { IconId::BootstrapCaretDownFill };

    let set_focused = {
        let is_focused = is_focused.clone();

        Callback::from(move |_: FocusEvent| {
            is_focused.set(true);
        })
    };

    let clear_focus = {
        let is_focused = is_focused.clone();

        Callback::from(move |_: FocusEvent| {
            is_focused.set(false);
        })
    };

    let items = items.iter()
        .enumerate()
        .map(|(index, item)| {
            let set_selected = {
                let on_change = on_change.clone();
                let items = items.clone();
                let selected_index = selected_index.clone();
                let is_focused = is_focused.clone();

                Callback::from(move |_: MouseEvent| {
                    selected_index.set(index);
                    is_focused.set(false);
                    on_change.emit(items[index].clone())
                })
            };

            html! {
                <div onmousedown={set_selected}>
                    <p>{item.to_string()}</p>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="select">
            <div class="content flex-row center-cross-axis" tabindex="-1" onfocus={set_focused} onfocusout={clear_focus}>
                <p class="flex-grow-1">{selected_item.to_string()}</p>
                <Icon icon_id={icon_id} class="fill-colour" width="15px" height="15px" />
            </div>
            if *is_focused {
                <div class="flex-column select-dropdown">
                    {items}
                </div>
            }
        </div>
    }
}