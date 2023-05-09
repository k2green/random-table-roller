use std::sync::Arc;

use common_data::{TableData, RollResult};
use regex::Regex;
use uuid::Uuid;
use web_sys::{HtmlTextAreaElement, HtmlInputElement};
use yew::prelude::*;

use crate::{hooks::prelude::UseTablesHandle, glue::{remove_table_with_callback, add_entries_with_callback, get_random_set_with_callback, remove_entry_with_callback, change_table_name_with_callback}, components::{modal::Modal, editable_header::EditableHeader, remove_button::RemoveButton}};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableTabsProps {
    pub tables: UseTablesHandle
}

#[function_component(TableTabs)]
pub fn table_tabs(props: &TableTabsProps) -> Html {
    let TableTabsProps { tables } = props.clone();

    let items = tables.tables()
        .iter()
        .enumerate()
        .map(|(idx, table)| {
            let set_index = {
                let tables = tables.clone();
                Callback::from(move |_: MouseEvent| {
                    tables.set_table_index(idx);
                })
            };

            let remove_table = {
                let tables = tables.clone();
                let id = tables.tables()[idx].id();

                Callback::from(move |_: MouseEvent| {
                    let tables = tables.clone();
                    remove_table_with_callback(id, move |_| {
                        tables.update()
                    });
                })
            };

            let class = match tables.get_selected_index() {
                Some(index) if index == idx => classes!("tab-button-disabled"),
                _ => classes!("tab-button")
            };

            html! {
                <div class={classes!(class, "flex-row")}>
                    <button onclick={set_index}>{table.name()}</button>
                    <RemoveButton on_click={remove_table} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="stretch flex-column no-scroll">
            <div class="flex-row scroll-x tab-bar">
                {items}
            </div>
            <div class="flex-grow-1 flex-column scroll tab-content">
                {get_table_content(tables)}
            </div>
        </div>
    }
}

fn get_table_content(tables: UseTablesHandle) -> Html {
    match tables.get_table_data() {
        None => render_welcome_content(),
        Some(table) => html! {
            <TabContent tables={tables} table={table} />
        }
    }
}

fn render_welcome_content() -> Html {
    html! {
        <div class="flex-column flex-grow-1">
            <h1 class="heading">{"Welcome!"}</h1>
            <p>{"This tool can be used to create random tables for your TTRPGS and use them to generate content."}</p>
            <p>{"To begin, use the menu on the right side of the screen to create a new table, or load a table from file. Once this is done the table will appear as a tab at the top of the window."}</p>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct TabContentProps {
    tables: UseTablesHandle,
    table: Arc<TableData>,
}

#[function_component(TabContent)]
fn tab_content(props: &TabContentProps) -> Html {
    let TabContentProps { tables, table } = props.clone();
    let is_add_entry_modal_open = use_state_eq(|| false);
    let is_roll_modal_open = use_state_eq(|| false);
    let is_roll_result_modal_open = use_state_eq(|| false);
    let roll_results = use_state_eq(|| Vec::<RollResult>::new());
    let id = table.id();

    let open_entries_modal = {
        let is_add_entry_modal_open = is_add_entry_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_add_entry_modal_open.set(true);
        })
    };

    let open_roll_modal = {
        let is_roll_modal_open = is_roll_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_roll_modal_open.set(true);
        })
    };

    let set_name = {
        let tables = tables.clone();

        Callback::from(move |name: String| {
            let tables = tables.clone();
            change_table_name_with_callback(id, name, move |_| {
                tables.update();
                tables.update_data();
            })
        })
    };

    let entries = table.iter()
        .enumerate()
        .map(|(index, entry)| {
            let remove_entry = {
                let tables = tables.clone();
                Callback::from(move |_: MouseEvent| {
                    let tables = tables.clone();
                    remove_entry_with_callback(id, index, move |_| {
                        tables.update_data();
                    });
                })
            };

            html! {
                <tr>
                    <td>{index + 1}</td>
                    <td>
                        <div class="flex-row min-height">
                            <p class="flex-grow-1">{entry}</p>
                            <RemoveButton on_click={remove_entry} />
                        </div>
                    </td>
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <>
            if *is_add_entry_modal_open {
                <AddEntryModal tables={tables.clone()} is_open={is_add_entry_modal_open} id={table.id()} />
            }
            if *is_roll_modal_open {
                <RandomRollModal is_open={is_roll_modal_open} is_results_open={is_roll_result_modal_open.clone()} results={roll_results.clone()} table={table.clone()} id={table.id()} />
            }
            if *is_roll_result_modal_open {
                <RollResults is_open={is_roll_result_modal_open} results={roll_results} />
            }
            <div class="flex-column flex-grow-1">
                <EditableHeader text={table.name().to_string()} callback={set_name} />
                <table>
                    <thead>
                        <tr>
                            <th>{"Roll"}</th>
                            <th>{"Entry"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {entries}
                    </tbody>
                </table>
            </div>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={open_entries_modal}>{"Add entries"}</button>
                <button class="flex-grow-1" onclick={open_roll_modal}>{"Roll"}</button>
            </div>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct RollResultsProps {
    is_open: UseStateHandle<bool>,
    results: UseStateHandle<Vec<RollResult>>,
}

#[function_component(RollResults)]
fn roll_results_modal(props: &RollResultsProps) -> Html {
    let RollResultsProps { is_open, results } = props.clone();

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
        })
    };

    let entries = results.iter()
        .map(|res| html! {
            <tr>
                <td class="align-right"><p>{format!("{}x", res.count())}</p></td>
                <td><p>{res.entry()}</p></td>
            </tr>
        })
        .collect::<Html>();

    html! {
        <Modal>
            <h3 class="heading">{"Results"}</h3>
            <div class="scroll flex-grow-1">
                <table class="stretch-width blank">
                    {entries}
                </table>
            </div>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={close}>{"Ok"}</button>
            </div>
        </Modal>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct RandomRollModalProps {
    is_open: UseStateHandle<bool>,
    is_results_open: UseStateHandle<bool>,
    results: UseStateHandle<Vec<RollResult>>,
    table: Arc<TableData>,
    id: Uuid
}

#[function_component(RandomRollModal)]
fn random_roll_modal(props: &RandomRollModalProps) -> Html {
    let RandomRollModalProps {
        is_open,
        is_results_open,
        results,
        table,
        id
    } = props.clone();

    let allow_duplicates = use_state_eq(|| true);
    let random_roll_count = use_state_eq(|| 1_usize);

    let make_rolls = {
        let is_open = is_open.clone();
        let is_results_open = is_results_open.clone();
        let results = results.clone();
        let random_roll_count = random_roll_count.clone();
        let allow_duplicates = allow_duplicates.clone();
        
        Callback::from(move |_: MouseEvent| {
            let is_open = is_open.clone();
            let is_results_open = is_results_open.clone();
            let results = results.clone();
            get_random_set_with_callback(id, *random_roll_count, *allow_duplicates, move |res| {
                is_open.set(false);
                is_results_open.set(true);
                results.set(res);
            });
        })
    };

    let update_allow_duplicates = {
        let random_roll_count = random_roll_count.clone();
        let allow_duplicates = allow_duplicates.clone();
        let table = table.clone();

        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            allow_duplicates.set(target.checked());

            if !target.checked() {
                random_roll_count.set((*random_roll_count).min(table.len()));
            }
        })
    };

    let update_count = {
        let random_roll_count = random_roll_count.clone();
        let allow_duplicates = allow_duplicates.clone();
        let table = table.clone();

        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            if let Ok(pattern) = Regex::new(r"^[ \n\r\t]*(\d+)[ \n\r\t]*$") {
                if let Some(captures) = pattern.captures(&target.value()) {
                    if let Some(capture) = captures.get(1) {
                        if let Ok(parsed) = usize::from_str_radix(capture.as_str(), 10) {
                            let mut new_value = parsed.max(1);

                            if !*allow_duplicates {
                                new_value = new_value.min(table.len());
                            }

                            random_roll_count.set(new_value);
                        }
                    }
                }
            }
        })
    };

    let cancel = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
        })
    };

    html! {
        <Modal>
            <h3 class="heading">{"Add entries"}</h3>
            <table class="stretch-width blank">
                <tr>
                    <td><p>{"Number of rolls:"}</p></td>
                    <td><input class="align-right" value={random_roll_count.to_string()} onchange={update_count} /></td>
                </tr>
                <tr>
                    <td><p>{"Allow duplicates:"}</p></td>
                    <td><input type="checkbox" checked={*allow_duplicates} onchange={update_allow_duplicates} /></td>
                </tr>
            </table>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={make_rolls}>{"Roll"}</button>
                <button class="flex-grow-1" onclick={cancel}>{"Cancel"}</button>
            </div>
        </Modal>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct AddEntryModalProps {
    is_open: UseStateHandle<bool>,
    tables: UseTablesHandle,
    id: Uuid
}

#[function_component(AddEntryModal)]
fn add_entry_modal(props: &AddEntryModalProps) -> Html {
    let AddEntryModalProps { is_open, tables, id } = props.clone();
    let text = use_state_eq(|| String::new());

    let update_text = {
        let text = text.clone();
        Callback::from(move |e: Event| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            text.set(target.value());
        })
    };

    let add_entries = {
        let is_open = is_open.clone();
        let text = text.clone();
        let tables = tables.clone();

        Callback::from(move |_: MouseEvent| {
            let is_open = is_open.clone();
            let tables = tables.clone();
            add_entries_with_callback(id, &*text, move |_| {
                tables.update_data();
                is_open.set(false);
            })
        })
    };

    let cancel = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
        })
    };

    html! {
        <Modal>
            <h3 class="heading">{"Add entries"}</h3>
            <p class="restrict-width">{"This is where you add new entries to your table. Multiple entries can be added by separating them onto new lines and empty lines will be ignored."}</p>
            <textarea onchange={update_text}>{&*text}</textarea>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={add_entries}>{"Add Entries"}</button>
                <button class="flex-grow-1" onclick={cancel}>{"Cancel"}</button>
            </div>
        </Modal>
    }
}