use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{components::{menu::Menu, table_tabs::TableTabs, modal::Modal}, hooks::prelude::*, glue::new_table_with_callback};

#[function_component(App)]
pub fn app() -> Html {
    let is_menu_open = use_state_eq(|| false);
    let is_new_table_modal_open = use_state_eq(|| false);
    let tables = use_tables();

    let open_new_table_modal = {
        let is_menu_open = is_menu_open.clone();
        let is_new_table_modal_open = is_new_table_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_menu_open.set(false);
            is_new_table_modal_open.set(true);
        })
    };

    html! {
        <>
            if *is_new_table_modal_open {
                <NewTableModal tables={tables.clone()} is_open={is_new_table_modal_open.clone()} is_menu_open={is_menu_open.clone()} />
            }
            <div class="flex-row stretch no-scroll">
                <Menu is_open={is_menu_open}>
                    <h2>{"Random table tool"}</h2>
                    <button onclick={open_new_table_modal}>{"New"}</button>
                </Menu>
                <main class="flex-grow-1 stretch-height no-scroll">
                    <TableTabs tables={tables.clone()} />
                </main>
            </div>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct NewTableModalProps {
    tables: UseTablesHandle,
    is_open: UseStateHandle<bool>,
    is_menu_open: UseStateHandle<bool>
}

#[function_component(NewTableModal)]
fn new_table_modal(props: &NewTableModalProps) -> Html {
    let NewTableModalProps { tables, is_open, is_menu_open } = props.clone();
    let table_name = use_state_eq(|| String::new());
    let entries_text = use_state_eq(|| String::new());

    let add_table = {
        let is_open = is_open.clone();
        let tables = tables.clone();
        let table_name = table_name.clone();
        let entries_text = entries_text.clone();

        Callback::from(move |_: MouseEvent| {
            let tables = tables.clone();
            let is_open = is_open.clone();

            new_table_with_callback(&*table_name, &*entries_text, move |_| {
                tables.update();
                is_open.set(false);
            });
        })
    };

    let update_name = {
        let table_name = table_name.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            table_name.set(target.value().trim().to_string());
        })
    };

    let update_entries = {
        let entries_text = entries_text.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            entries_text.set(target.value().trim().to_string());
        })
    };

    let cancel = {
        let is_open = is_open.clone();
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
            is_menu_open.set(true);
        })
    };

    html! {
        <Modal>
            <h3 class="heading">{"New Table"}</h3>
            <div class="flex-row">
                <p>{"Table Name:"}</p>
                <input class="flex-grow-1" value={(*table_name).clone()} onchange={update_name} />
            </div>
            <p>{"Initial entries:"}</p>
            <textarea onchange={update_entries}>{(*entries_text).clone()}</textarea>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={add_table}>{"Add Table"}</button>
                <button class="flex-grow-1" onclick={cancel}>{"Cancel"}</button>
            </div>
        </Modal>
    }
}