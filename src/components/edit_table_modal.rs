use std::{sync::Arc, ops::Deref};

use common_data::{TableData, Currency, TableEntry};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{components::{full_page_modal::FullPageModal, number_field::NumberField, currency_field::CurrencyField, remove_button::RemoveButton, checkbox::Checkbox}, hooks::prelude::*, glue::update_table_with_callback};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditTableModalProps {
    pub table: Arc<TableData>,
    #[prop_or_default]
    pub on_update: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<MouseEvent>
}

#[function_component(EditTableModal)]
pub fn edit_table_modal(props: &EditTableModalProps) -> Html {
    let EditTableModalProps { table, on_update, on_cancel } = props.clone();
    let is_add_modal_open = use_state_eq(|| false);
    let use_cost = use_state_eq(|| table.use_cost());
    let use_weight = use_state_eq(|| table.use_weight());
    let entries = use_vec_state_eq(|| table.entries().clone());

    let show_modal = {
        let is_add_modal_open = is_add_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_add_modal_open.set(true);
        })
    };

    let hide_modal = {
        let is_add_modal_open = is_add_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_add_modal_open.set(false);
        })
    };

    let update_entries = {
        let entries = entries.clone();
        let is_add_modal_open = is_add_modal_open.clone();
        Callback::from(move |new_entries: Vec<TableEntry>| {
            entries.insert_all(new_entries);
            is_add_modal_open.set(false);
        })
    };

    html! {
        if *is_add_modal_open {
            <AddEntryModal use_cost={*use_cost} use_weight={*use_weight} on_complete={update_entries} on_cancel={hide_modal} />
        } else {
            <EditTableModalContent table={table} use_cost={use_cost} use_weight={use_weight} entries={entries} on_update={on_update} on_cancel={on_cancel} on_open_add_entries={show_modal} />
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct EditTableModalContentProps {
    pub table: Arc<TableData>,
    pub entries: UseVecStateHandle<TableEntry>,
    pub use_cost: UseStateHandle<bool>,
    pub use_weight: UseStateHandle<bool>,
    #[prop_or_default]
    pub on_update: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_open_add_entries: Callback<MouseEvent>
}

#[function_component(EditTableModalContent)]
fn edit_table_modal_content(props: &EditTableModalContentProps) -> Html {
    let EditTableModalContentProps {
        table,
        entries,
        use_cost,
        use_weight,
        on_update,
        on_cancel,
        on_open_add_entries
    } = props.clone();

    let name = use_state_eq(|| table.name().to_string());
    let is_update_disabled = entries.len() == 0;

    let update_name = {
        let name = name.clone();

        Callback::from(move |e: Event| {
            let old_name = name.deref().clone();
            let target: HtmlInputElement = e.target_unchecked_into();
            let target_value = target.value();

            if !target_value.trim().is_empty() {
                name.set(target_value.trim().to_string());
            } else {
                target.set_value(&old_name);
            }
        })
    };

    let update_use_cost = {
        let use_cost = use_cost.clone();
        Callback::from(move |checked: bool| {
            use_cost.set(checked);
        })
    };

    let update_use_weight = {
        let use_weight = use_weight.clone();
        Callback::from(move |checked: bool| {
            use_weight.set(checked);
        })
    };

    let update_table = {
        let on_update = on_update.clone();
        let table = table.clone();
        let name = name.clone();
        let use_cost = use_cost.clone();
        let use_weight = use_weight.clone();
        let entries = entries.clone();

        Callback::from(move |_: MouseEvent| {
            let on_update = on_update.clone();
            let name = some_if_different(table.name().to_string(), name.deref().clone());
            let use_cost = some_if_different(table.use_cost(), *use_cost);
            let use_weight = some_if_different(table.use_weight(), *use_weight);
            let entries = some_if_different(table.entries().clone(), entries.deref().clone());

            update_table_with_callback(table.id(), name, use_cost, use_weight, entries, move |_: ()| {
                on_update.emit(());
            });
        })
    };

    let entry_items = entries.iter()
        .enumerate()
        .map(|(index, entry)| {
            let update_name = {
                let old_name = entry.name().to_string();
                let entries = entries.clone();
                Callback::from(move |e: Event| {
                    let target: HtmlInputElement = e.target_unchecked_into();
                    let target_value = target.value();
        
                    if !target_value.trim().is_empty() {
                        entries.update_single(index, |old| {
                            let mut new = old.clone();
                            new.set_name(target_value.trim());
                            new
                        });
                    } else {
                        target.set_value(&old_name);
                    }
                })
            };

            let update_weight = {
                let entries = entries.clone();
                Callback::from(move |weight: usize| {
                    entries.update_single(index, |old| {
                        let mut new = old.clone();
                        new.set_weight(weight);
                        new
                    });
                })
            };

            let update_cost = {
                let entries = entries.clone();
                Callback::from(move |cost: Currency| {
                    entries.update_single(index, |old| {
                        let mut new = old.clone();
                        new.set_cost(cost);
                        new
                    });
                })
            };

            let remove_entry = {
                let entries = entries.clone();
                Callback::from(move |_: MouseEvent| {
                    entries.remove(index);
                })
            };

            let validate_weight = Callback::from(move |weight: usize| {
                weight.clamp(1, 100)
            });

            html! {
                <div class="flex-row">
                    <input class="flex-grow-1" value={entry.name().to_string()} onchange={update_name} />
                    if *use_weight {
                        <NumberField<usize> title="Weight" class="number" value={entry.weight()} validate={validate_weight} on_change={update_weight} />
                    }
                    if *use_cost {
                        <CurrencyField title="Cost" on_change={update_cost} />
                    }
                    <RemoveButton on_click={remove_entry} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <FullPageModal>
                <table class="stretch-width settings">
                    <tr>
                        <td><p>{"Table Name:"}</p></td>
                        <td><input class="flex-grow-1" value={(*name).clone()} onchange={update_name} /></td>
                    </tr>
                    <tr>
                        <td><p class="flex-grow-1">{"Use costs:"}</p></td>
                        <td><Checkbox class="stretch-height flex-row center-cross-axis end-main-axis" checked={*use_cost} on_change={update_use_cost} /></td>
                    </tr>
                    <tr>
                        <td><p class="flex-grow-1">{"Use weights:"}</p></td>
                        <td><Checkbox class="stretch-height flex-row center-cross-axis end-main-axis" checked={*use_weight} on_change={update_use_weight} /></td>
                    </tr>
                </table>
                <div class="flex-column flex-grow-1 table-style">
                    <h2>{"Table entries"}</h2>
                    <div class="flex-column content">
                        {entry_items}
                    </div>
                </div>
                <div class="flex-row button-row">
                    <button class="flex-grow-1" onclick={update_table} disabled={is_update_disabled}>{"Update table"}</button>
                    <button class="flex-grow-1" onclick={on_open_add_entries}>{"Add new entries"}</button>
                    <button class="flex-grow-1" onclick={on_cancel}>{"Cancel"}</button>
                </div>
            </FullPageModal>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct AddEntryModalProps {
    use_cost: bool,
    use_weight: bool,
    #[prop_or_default]
    on_complete: Callback<Vec<TableEntry>>,
    #[prop_or_default]
    on_cancel: Callback<MouseEvent>,
}

#[function_component(AddEntryModal)]
fn add_entry_modal(props: &AddEntryModalProps) -> Html {
    let AddEntryModalProps { use_cost, use_weight, on_complete, on_cancel } = props.clone();
    let entries = use_vec_state(|| Vec::<TableEntry>::new());
    let disable_add = entries.len() == 0 || entries.iter().all(|e| e.name().trim().is_empty());

    let add_entries = {
        let on_complete = on_complete.clone();
        let entries = entries.clone();

        Callback::from(move |_: MouseEvent| {
            on_complete.emit(entries.deref().clone());
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

            let weight_changed = {
                let entries = entries.clone();
                Callback::from(move |weight: usize| {
                    entries.update(move |entry_index, old| if entry_index == index {
                        let mut new = old.clone();
                        new.set_weight(weight);
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

            let validate_weight = Callback::from(move |weight: usize| {
                weight.clamp(1, 100)
            });

            html! {
                <div class="flex-row">
                    <input class="flex-grow-1" value={entry.name().to_string()} onchange={update_entry} />
                    if use_weight {
                        <NumberField<usize> title="Weight" class="number" value={entry.weight()} validate={validate_weight} on_change={weight_changed} />
                    }
                    if use_cost {
                        <CurrencyField title="Cost" on_change={currency_changed} />
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
                <button class="flex-grow-1" onclick={on_cancel}>{"Cancel"}</button>
            </div>
        </FullPageModal>
    }
}

fn some_if_different<T: Eq>(original: T, new: T) -> Option<T> {
    if new == original {
        None
    } else {
        Some(new)
    }
}