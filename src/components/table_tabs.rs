use std::sync::Arc;

use common_data::{TableData, RollResult, TableEntry, RollType};
use yew::prelude::*;

use crate::{hooks::prelude::UseTablesHandle, glue::*, components::{remove_button::RemoveButton, roll_modals::{RollTypeSelectionModal, RollByCountModal, RollByCostModal, RollResultsModal}, edit_table_modal::EditTableModal}};

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

fn entry_row(index: usize, entry: &TableEntry, table: Arc<TableData>) -> Html {
    let use_weight = table.use_weight();
    let use_cost = table.use_cost();
    
    html! {
        <tr>
            <td>{index + 1}</td>
            <td><p class="flex-grow-1">{entry.name()}</p></td>
            if use_weight {
                <td><p class="flex-grow-1">{entry.weight().to_string()}</p></td>
            }
            if use_cost {
                <td><p class="flex-grow-1">{entry.cost().to_string()}</p></td>
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
    let is_edit_modal_open = use_state_eq(|| false);
    let is_roll_modal_open = use_state_eq(|| false);
    let id = table.id();

    let open_roll_modal = {
        let is_roll_modal_open = is_roll_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_roll_modal_open.set(true);
        })
    };

    let open_edit_modal = {
        let is_edit_modal_open = is_edit_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_edit_modal_open.set(true);
        })
    };

    let close_edit_modal = {
        let is_edit_modal_open = is_edit_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_edit_modal_open.set(false);
        })
    };

    let on_table_update = {
        let is_edit_modal_open = is_edit_modal_open.clone();
        let tables = tables.clone();
        Callback::from(move |_: ()| {
            tables.update_data();
            is_edit_modal_open.set(false);
        })
    };

    let entries = table.iter()
        .enumerate()
        .map(|(index, entry)| entry_row(index, entry, table.clone()))
        .collect::<Html>();

    html! {
        <>
            if *is_edit_modal_open {
                <EditTableModal table={table.clone()} on_update={on_table_update} on_cancel={close_edit_modal} />
            }
            if *is_roll_modal_open {
                <RandomRollModal table={table.clone()} use_cost={table.use_cost()} is_open={is_roll_modal_open} />
            }
            <div class="flex-column flex-grow-1">
                <h2 class="heading">{table.name().to_string()}</h2>
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
                <button class="flex-grow-1" onclick={open_edit_modal}>{"Edit table"}</button>
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