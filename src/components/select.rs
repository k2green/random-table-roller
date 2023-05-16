use std::{fmt::Display, sync::Arc};

use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectProps<T: Clone + PartialEq + Display + 'static> {
    pub items: Arc<Vec<T>>,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub parent_class: Classes,
    #[prop_or_default]
    pub select_class: Classes,
    #[prop_or_default]
    pub on_change: Callback<T>
}

#[function_component(Select)]
pub fn select<T: Clone + PartialEq + Display + 'static>(props: &SelectProps<T>) -> Html {
    let SelectProps { parent_class, title, select_class, items, on_change } = props.clone();
    let selected_item = use_state_eq(|| items[0].clone());
    
    html! {
        <SelectDirect<T> items={items} selected_item={selected_item} title={title} parent_class={parent_class} select_class={select_class} on_change={on_change} />
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectDirectProps<T: Clone + PartialEq + Display + 'static> {
    pub selected_item: UseStateHandle<T>,
    pub items: Arc<Vec<T>>,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub parent_class: Classes,
    #[prop_or_default]
    pub select_class: Classes,
    #[prop_or_default]
    pub on_change: Callback<T>
}

#[function_component(SelectDirect)]
pub fn select_direct<T: Clone + PartialEq + Display + 'static>(props: &SelectDirectProps<T>) -> Html {
    let SelectDirectProps { selected_item, parent_class, title, select_class, items, on_change } = props.clone();
    let is_focused = use_state_eq(|| false);
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
                let selected_item = selected_item.clone();
                let is_focused = is_focused.clone();

                Callback::from(move |_: MouseEvent| {
                    let item = &items[index];
                    selected_item.set(item.clone());
                    is_focused.set(false);
                    on_change.emit(item.clone())
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
        <div class={classes!(parent_class, "select")}>
            <div title={title} class={classes!(select_class, "content", "flex-row", "center-cross-axis")} tabindex="-1" onfocus={set_focused} onfocusout={clear_focus}>
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