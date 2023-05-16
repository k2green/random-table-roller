use std::sync::Arc;

use common_data::{RollResult, Currency, RollType, RollLimit};
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{components::{modal::Modal, currency_field::CurrencyFieldDirect, full_page_modal::FullPageModal, number_field::NumberField}, glue::get_random_set_with_callback, hooks::prelude::use_currency_state_eq};

fn get_roll_type_html(roll_type: RollType, on_select: Callback<RollType>) -> Html {
    let select = {
        let on_select = on_select.clone();
        let value = roll_type.clone();
        Callback::from(move |_: MouseEvent| {
            on_select.emit(value);
        })
    };

    html! {
        <div class="flex-row button-row">
            <button class="flex-grow-1" onclick={select}>{format!("By {}", roll_type)}</button>
        </div>
    }
}

fn get_roll_types_html(on_select: Callback<RollType>) -> Html {
    RollType::get_values().into_iter()
        .map(|v| get_roll_type_html(v, on_select.clone()))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RollTypeSelectionModalProps {
    pub on_select: Callback<RollType>,
    pub on_cancel: Callback<MouseEvent>,
}

#[function_component(RollTypeSelectionModal)]
pub fn roll_type_selection_modal(props: &RollTypeSelectionModalProps) -> Html {
    let RollTypeSelectionModalProps { on_select, on_cancel } = props.clone();

    html! {
        <Modal>
            <h3 class="header">{"Roll Type"}</h3>
            <p class="restrict-width">{"Use these buttons to choose how you want to roll. Rolling by count allows you to select a number of items to roll by, rolling by cost allows you to roll using a maximum total cost."}</p>
            {get_roll_types_html(on_select)}
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={on_cancel}>{"Cancel"}</button>
            </div>
        </Modal>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RollByCountModalProps {
    pub table_id: Uuid,
    pub max_count: usize,
    pub on_complete: Callback<Vec<RollResult>>,
    pub on_cancel: Callback<MouseEvent>,
}

#[function_component(RollByCountModal)]
pub fn roll_by_count_modal(props: &RollByCountModalProps) -> Html {
    let RollByCountModalProps { table_id, max_count, on_complete, on_cancel } = props.clone();
    let count = use_state_eq(|| 1_usize);
    let allow_duplicates = use_state_eq(|| true);
    let max = if *allow_duplicates { None } else { Some(max_count) };

    let update_count = {
        let count = count.clone();
        Callback::from(move |value: usize| {
            count.set(value);
        })
    };

    let update_allow_duplicates = {
        let allow_duplicates = allow_duplicates.clone();
        let count = count.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let checked = target.checked();
            allow_duplicates.set(checked);

            if !checked {
                count.set(clamp_count(*count, 1, Some(max_count)));
            }
        })
    };

    let on_complete = {
        let count = count.clone();
        let allow_duplicates = allow_duplicates.clone();
        let on_complete = on_complete.clone();

        Callback::from(move |_: MouseEvent| {
            let on_complete = on_complete.clone();
            get_random_set_with_callback(table_id, RollLimit::Count(*count), *allow_duplicates, move |results| {
                on_complete.emit(results);
            })
        })
    };

    let validate_count = Callback::from(move |amount: usize| clamp_count(amount, 1, max));

    html! {
        <Modal>
            <h3 class="header">{"Roll by count"}</h3>
            <table class="stretch-width blank">
                <tr>
                    <td>{"Count:"}</td>
                    <NumberField<usize> class="number" get_default={|_: ()| 1_usize} value={*count} validate={validate_count} on_change={update_count} />
                </tr>
                <tr>
                    <td>{"Allow duplicates:"}</td>
                    <input type="checkbox" checked={*allow_duplicates} onchange={update_allow_duplicates} />
                </tr>
            </table>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={on_complete}>{"Roll"}</button>
                <button class="flex-grow-1" onclick={on_cancel}>{"Cancel"}</button>
            </div>
        </Modal>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RollByCostModalProps {
    pub table_id: Uuid,
    pub max_cost: Currency,
    pub on_complete: Callback<Vec<RollResult>>,
    pub on_cancel: Callback<MouseEvent>,
}

#[function_component(RollByCostModal)]
pub fn roll_by_cost_modal(props: &RollByCostModalProps) -> Html {
    let RollByCostModalProps { table_id, max_cost, on_complete, on_cancel } = props.clone();
    let cost = use_currency_state_eq(|| Currency::Copper(1));
    let allow_duplicates = use_state_eq(|| true);
    let max = if *allow_duplicates { None } else { Some(max_cost) };

    let update_cost = {
        let cost = cost.clone();
        let max = max.clone();
        Callback::from(move |value: Currency| {
            cost.set(clamp_cost(value, Currency::Copper(1), max));
        })
    };

    let update_allow_duplicates = {
        let cost = cost.clone();
        let allow_duplicates = allow_duplicates.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let checked = target.checked();
            allow_duplicates.set(checked);

            if !checked {
                cost.set(clamp_cost(cost.currency(), Currency::Copper(1), Some(max_cost)));
            }
        })
    };

    let on_complete = {
        let cost = cost.clone();
        let allow_duplicates = allow_duplicates.clone();
        let on_complete = on_complete.clone();

        Callback::from(move |_: MouseEvent| {
            let on_complete = on_complete.clone();
            get_random_set_with_callback(table_id, RollLimit::Cost(cost.currency()), *allow_duplicates, move |results| {
                on_complete.emit(results);
            })
        })
    };

    html! {
        <Modal>
            <h3 class="header">{"Roll by count"}</h3>
            <table class="stretch-width blank">
                <tr>
                    <td>{"Count:"}</td>
                    <CurrencyFieldDirect amount={cost.amount_handle()} currency_type={cost.currency_type_handle()} on_change={update_cost} />
                </tr>
                <tr>
                    <td>{"Allow duplicates:"}</td>
                    <input type="checkbox" checked={*allow_duplicates} onchange={update_allow_duplicates} />
                </tr>
            </table>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={on_complete}>{"Roll"}</button>
                <button class="flex-grow-1" onclick={on_cancel}>{"Cancel"}</button>
            </div>
        </Modal>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RollResultsModalProps {
    pub show_cost: bool,
    pub results: Arc<Vec<RollResult>>,
    pub on_close: Callback<MouseEvent>
}

#[function_component(RollResultsModal)]
pub fn roll_results_modal(props: &RollResultsModalProps) -> Html {
    let RollResultsModalProps { show_cost, results, on_close } = props.clone();

    let result_rows = results.iter()
        .map(|result| {
            let count = result.count();
            let cost = result.entry().cost();
            let total_cost = Currency::from(cost.to_copper().amount() * count as u128);

            html! {
                <tr>
                    <td>{format!("{}x", count)}</td>
                    <td>{result.entry().name()}</td>
                    if show_cost {
                        <td>{cost.to_string()}</td>
                        <td>{total_cost.to_string()}</td>
                    }
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <FullPageModal>
            <h2 class="heading">{"Results"}</h2>
            <table class="stretch-width">
                <thead>
                    <tr>
                        <th>{"Amount"}</th>
                        <th>{"Result"}</th>
                        if show_cost {
                            <th>{"Cost"}</th>
                            <th>{"Total Cost"}</th>
                        }
                    </tr>
                </thead>
                <tbody>
                    {result_rows}
                </tbody>
            </table>
            <div class="flex-row button-row">
                <button class="flex-grow-1" onclick={on_close}>{"Ok"}</button>
            </div>
        </FullPageModal>
    }
}

fn clamp_count(value: usize, min: usize, max: Option<usize>) -> usize {
    match max {
        Some(max) => value.clamp(min, max),
        None => value.max(min)
    }
}

fn clamp_cost(value: Currency, min: Currency, max: Option<Currency>) -> Currency {
    match max {
        Some(max) => value.clamp(min, max),
        None => value.max(min)
    }
}