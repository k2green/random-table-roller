use std::sync::Arc;

use common_data::{TableData, RollResult, TableEntry, Currency, RollType};
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{hooks::prelude::{UseTablesHandle, use_vec_state}, glue::*, components::{editable_header::EditableHeader, remove_button::RemoveButton, full_page_modal::FullPageModal, currency_field::CurrencyField, roll_modals::{RollTypeSelectionModal, RollByCountModal, RollByCostModal, RollResultsModal}}};

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

fn entry_row(index: usize, entry: &TableEntry, id: Uuid, tables: UseTablesHandle, table: Arc<TableData>) -> Html {
    let remove_entry = {
        let tables = tables.clone();
        Callback::from(move |_: MouseEvent| {
            let tables = tables.clone();
            remove_entry_with_callback(id, index, move |_| {
                tables.update_data();
            });
        })
    };

    let use_weight = table.use_weight();
    let use_cost = table.use_cost();
    
    html! {
        <tr>
            <td>{index + 1}</td>
            <td>
                <div class="flex-row min-height">
                    <p class="flex-grow-1">{entry.name()}</p>
                    if !use_weight && !use_cost {
                        <RemoveButton on_click={remove_entry.clone()} />
                    }
                </div>
            </td>
            if use_weight {
                <td>
                    <div class="flex-row min-height">
                        <p class="flex-grow-1">{entry.weight().to_string()}</p>
                        if !use_cost {
                            <RemoveButton on_click={remove_entry.clone()} />
                        }
                    </div>
                </td>
            }
            if use_cost {
                <td>
                    <div class="flex-row min-height">
                        <p class="flex-grow-1">{entry.cost().to_string()}</p>
                        <RemoveButton on_click={remove_entry.clone()} />
                    </div>
                </td>
            }
        </tr>
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
        .map(|(index, entry)| entry_row(index, entry, id, tables.clone(), table.clone()))
        .collect::<Html>();

    html! {
        <>
            if *is_add_entry_modal_open {
                <AddEntryModal tables={tables.clone()} is_open={is_add_entry_modal_open} id={table.id()} />
            }
            if *is_roll_modal_open {
                <RandomRollModal table={table.clone()} use_cost={table.use_cost()} is_open={is_roll_modal_open} />
            }
            <div class="flex-column flex-grow-1">
                <EditableHeader text={table.name().to_string()} callback={set_name} />
                <table>
                    <thead>
                        <tr>
                            <th>{"Roll"}</th>
                            <th>{"Entry"}</th>
                            if table.use_weight() {
                                <th>{"Weight"}</th>
                            }
                            if table.use_cost() {
                                <th>{"Cost"}</th>
                            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RollModal {
    SelectMode,
    RollByCount,
    RollByCost,
    Results
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct RandomRollModalProps {
    table: Arc<TableData>,
    use_cost: bool,
    is_open: UseStateHandle<bool>
}

#[function_component(RandomRollModal)]
fn random_roll_modal(props: &RandomRollModalProps) -> Html {
    let RandomRollModalProps { table, use_cost, is_open } = props.clone();
    let current_modal = use_state_eq(|| if use_cost { RollModal::SelectMode } else { RollModal::RollByCount });
    let results = use_state_eq(|| Arc::new(Vec::<RollResult>::new()));

    let select_roll_type = {
        let current_modal = current_modal.clone();
        Callback::from(move |rt: RollType| {
            current_modal.set(match rt {
                RollType::Count => RollModal::RollByCount,
                RollType::Cost => RollModal::RollByCost,
            });
        })
    };

    let close_modal = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
        })
    };

    let complete = {
        let results = results.clone();
        let current_modal = current_modal.clone();
        Callback::from(move |res: Vec<RollResult>| {
            results.set(Arc::new(res));
            current_modal.set(RollModal::Results);
        })
    };

    match &*current_modal {
        RollModal::SelectMode => html! { <RollTypeSelectionModal on_select={select_roll_type} on_cancel={close_modal.clone()} /> },
        RollModal::RollByCount => html! { <RollByCountModal table={table.clone()} max_count={table.len()} on_complete={complete} on_cancel={close_modal} /> },
        RollModal::RollByCost => html! { <RollByCostModal table={table.clone()} max_cost={table.total_cost()} on_complete={complete} on_cancel={close_modal} /> },
        RollModal::Results => html! { <RollResultsModal show_cost={use_cost} results={(*results).clone()} on_close={close_modal} /> },
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
    let entries = use_vec_state(|| Vec::<TableEntry>::new());
    let disable_add = entries.len() == 0 || entries.iter().all(|e| e.name().trim().is_empty());
    let table = tables.get_table_data().unwrap();

    let add_entries = {
        let is_open = is_open.clone();
        let entries = entries.clone();
        let tables = tables.clone();

        Callback::from(move |_: MouseEvent| {
            let is_open = is_open.clone();
            let tables = tables.clone();
            add_entries_with_callback(id, (*entries).clone(), move |_| {
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

    let insert_new = {
        let entries = entries.clone();
        Callback::from(move |_: MouseEvent| {
            entries.insert(TableEntry::new(Currency::Copper(1)));
        })
    };

    let entry_items = entries.iter()
        .enumerate()
        .map(|(index, entry)| {
            let update_entry = {
                let entries = entries.clone();
                Callback::from(move |e: Event| {
                    let target: HtmlInputElement = e.target_unchecked_into();
                    let new_entry = target.value();
                    entries.update(move |entry_index, old| if entry_index == index {
                        let mut new = old.clone();
                        new.set_name(new_entry.trim());
                        new
                    } else {
                        old.clone()
                    })
                })
            };

            let currency_changed = {
                let entries = entries.clone();
                Callback::from(move |c: Currency| {
                    entries.update(move |entry_index, old| if entry_index == index {
                        let mut new = old.clone();
                        new.set_cost(c);
                        new
                    } else {
                        old.clone()
                    })
                })
            };

            let remove_entry = {
                let entries = entries.clone();
                Callback::from(move |_: MouseEvent| {
                    entries.remove(index);
                })
            };

            html! {
                <div class="flex-row">
                    <input class="flex-grow-1" value={entry.name().to_string()} onchange={update_entry} />
                    if table.use_cost() {
                        <CurrencyField on_change={currency_changed} />
                    }
                    <RemoveButton on_click={remove_entry} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <FullPageModal>
            <h3 class="heading">{"Add entries"}</h3>
            <p>{"Here you can add new entries to your table. Use the '+' button to add a new entry."}</p>
            <div class="flex-column flex-grow-1 table-style">
                <h2>{"Table entries"}</h2>
                <div class="flex-column content">
                    {entry_items}
                </div>
            </div>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={insert_new}>{"+"}</button>
            </div>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={add_entries} disabled={disable_add}>{"Add Entries"}</button>
                <button class="flex-grow-1" onclick={cancel}>{"Cancel"}</button>
            </div>
        </FullPageModal>
    }
}