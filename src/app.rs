use std::path::PathBuf;

use common_data::{TableEntry, Currency};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{components::{menu::Menu, table_tabs::TableTabs, full_page_modal::FullPageModal, remove_button::RemoveButton, currency_field::CurrencyField, number_field::NumberField}, hooks::prelude::*, glue::*};

fn save_table(is_menu_open: UseStateHandle<bool>, tables: UseTablesHandle) {
    let is_menu_open = is_menu_open.clone();
    let tables = tables.clone();
    
    if let Some(table) = tables.get_table_data() {
        if let Some(path) = table.path() {
            save_table_with_callback(table.id(), path.to_path_buf(), move |_| {
                is_menu_open.set(false);
                log::info!("Saved table to {:?}", path);
            });
        }
    } 
}

fn save_table_as(is_menu_open: UseStateHandle<bool>, tables: UseTablesHandle) {
    let is_menu_open = is_menu_open.clone();
    let tables = tables.clone();
    
    get_save_table_path_with_callback(move |value: Option<PathBuf>| {
        let is_menu_open = is_menu_open.clone();
        let tables = tables.clone();
        if let (Some(table), Some(path)) = (tables.get_table_data(), value) {
            save_table_with_callback(table.id(), path.clone(), move |_| {
                is_menu_open.set(false);
                tables.update_data();
                log::info!("Saved table to {:?}", path);
            });
        }
    })
}

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

    let save_table = {
        let is_menu_open = is_menu_open.clone();
        let tables = tables.clone();

        Callback::from(move |_: MouseEvent| save_table(is_menu_open.clone(), tables.clone()))
    };

    let save_table_as = {
        let is_menu_open = is_menu_open.clone();
        let tables = tables.clone();

        Callback::from(move |_: MouseEvent| save_table_as(is_menu_open.clone(), tables.clone()))
    };

    let open_table = {
        let is_menu_open = is_menu_open.clone();
        let tables = tables.clone();

        Callback::from(move |_: MouseEvent| {
            let is_menu_open = is_menu_open.clone();
            let tables = tables.clone();
            
            get_open_table_path_with_callback(move |value: Option<PathBuf>| {
                let is_menu_open = is_menu_open.clone();
                let tables = tables.clone();
                
                if let Some(path) = value {
                    open_table_with_callback(path, move |_| {
                        tables.update();
                        is_menu_open.set(false);
                    });
                }
            })
        })
    };

    let is_save_disabled = match tables.get_table_data() {
        Some(data) => data.path().is_none(),
        _ => true
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
                    <button onclick={save_table} disabled={is_save_disabled}>{"Save"}</button>
                    <button onclick={save_table_as} disabled={tables.get_selected_index().is_none()}>{"Save As"}</button>
                    <button onclick={open_table}>{"Open"}</button>
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
    let entries = use_vec_state(|| Vec::<TableEntry>::new());
    let use_cost = use_state_eq(|| false);
    let use_weight = use_state_eq(|| false);
    let disable_add_button = table_name.trim().is_empty() || (entries.len() > 0 && entries.iter().any(|e| e.name().trim().is_empty()));

    let update_use_cost = {
        let use_cost = use_cost.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            use_cost.set(target.checked());
        })
    };

    let update_use_weight = {
        let use_weight = use_weight.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            use_weight.set(target.checked());
        })
    };

    let add_table = {
        let is_open = is_open.clone();
        let use_cost = use_cost.clone();
        let use_weight = use_weight.clone();
        let tables = tables.clone();
        let table_name = table_name.clone();
        let entries = entries.clone();

        Callback::from(move |_: MouseEvent| {
            let tables = tables.clone();
            let is_open = is_open.clone();
            let name = (*table_name).trim();
            let entries = (*entries).clone();

            if !name.is_empty() {
                log::info!("Entries: {:?}", &entries);
                new_table_with_callback(*use_cost, *use_weight, name.to_string(), entries, move |_| {
                    tables.update();
                    is_open.set(false);
                });
            }
        })
    };

    let update_name = {
        let table_name = table_name.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            table_name.set(target.value().trim().to_string());
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

            let remove_entry = {
                let entries = entries.clone();
                Callback::from(move |_: MouseEvent| {
                    entries.remove(index);
                })
            };

            let currency_changed = {
                let entries = entries.clone();
                Callback::from(move |c: Currency| {
                    entries.update(move |current, old| {
                        if current == index {
                            let mut new = old.clone();
                            new.set_cost(c);
                            new
                        } else {
                            old.clone()
                        }
                    });
                })
            };

            let set_weight = {
                let entries = entries.clone();
                Callback::from(move |weight: usize| {
                    entries.update(move |current_idx, old| {
                        let mut new = old.clone();
                        if current_idx == index {
                            new.set_weight(weight);
                        }

                        new
                    })
                })
            };

            let validate_weight = Callback::from(move |weight: usize| {
                weight.clamp(1, 100)
            });

            html! {
                <div class="flex-row">
                    <input class="flex-grow-1" value={entry.name().to_string()} onchange={update_entry} />
                    if *use_weight {
                        <NumberField<usize> class="number" value={entry.weight()} validate={validate_weight} on_change={set_weight} />
                    }
                    if *use_cost {
                        <CurrencyField on_change={currency_changed} />
                    }
                    <RemoveButton on_click={remove_entry} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <FullPageModal>
            <h3 class="heading">{"New Table"}</h3>
            <div class="flex-row">
                <p class="flex-grow-1">{"Use costs:"}</p>
                <input type="checkbox" checked={*use_cost} onchange={update_use_cost} />
            </div>
            <div class="flex-row">
                <p class="flex-grow-1">{"Use weights:"}</p>
                <input type="checkbox" checked={*use_weight} onchange={update_use_weight} />
            </div>
            <div class="flex-row">
                <p>{"Table Name:"}</p>
                <input class="flex-grow-1" value={(*table_name).clone()} onchange={update_name} />
            </div>
            <p class="vert-margin">{"Below you can add the initial entries in the table. Multiple entries can be added by splitting them into multiple lines."}</p>
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
                <button class="flex-grow-1" onclick={add_table} disabled={disable_add_button}>{"Add Table"}</button>
                <button class="flex-grow-1" onclick={cancel}>{"Cancel"}</button>
            </div>
        </FullPageModal>
    }
}