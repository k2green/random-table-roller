use std::str::FromStr;

use common_data::Currency;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::try_parse_number;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CurrencyType {
    Platinum,
    Gold,
    Silver,
    Copper,
}

impl std::fmt::Display for CurrencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyType::Platinum => write!(f, "pp"),
            CurrencyType::Gold => write!(f, "gp"),
            CurrencyType::Silver => write!(f, "sp"),
            CurrencyType::Copper => write!(f, "cp"),
        }
    }
}

impl FromStr for CurrencyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pp" => Ok(CurrencyType::Platinum),
            "gp" => Ok(CurrencyType::Gold),
            "sp" => Ok(CurrencyType::Silver),
            "cp" => Ok(CurrencyType::Copper),
            _ => Err(())
        }
    }
}

fn get_currency(amount: usize, currency_type: CurrencyType) -> Currency {
    match currency_type {
        CurrencyType::Platinum => Currency::Platinum(amount),
        CurrencyType::Gold => Currency::Gold(amount),
        CurrencyType::Silver => Currency::Silver(amount),
        CurrencyType::Copper => Currency::Copper(amount),
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CurrencyFieldProps {
    #[prop_or_default]
    container_class: Classes,
    #[prop_or_default]
    field_class: Classes,
    on_change: Callback<Currency>
}

#[function_component(CurrencyField)]
pub fn currency_field(props: &CurrencyFieldProps) -> Html {
    let CurrencyFieldProps { container_class, field_class, on_change } = props.clone();
    let amount = use_state_eq(|| 0_usize);
    let currency_type = use_state_eq(|| CurrencyType::Copper);

    let update_amount = {
        let amount = amount.clone();
        let currency_type = currency_type.clone();
        let on_change = on_change.clone();

        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let target_value = target.value();
            if let Some(parsed) = try_parse_number(&target_value) {
                let parsed = parsed.min(1);
                amount.set(parsed);
                on_change.emit(get_currency(parsed, *currency_type));
            }
        })
    };

    let update_type = {
        let amount = amount.clone();
        let currency_type = currency_type.clone();
        let on_change = on_change.clone();

        Callback::from(move |e: Event| {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let target_value = target.value();
            if let Ok(new_type) = CurrencyType::from_str(&target_value) {
                currency_type.set(new_type);
                on_change.emit(get_currency(*amount, new_type));
            }
        })
    };

    html! {
        <div class={classes!(container_class, "flex-row")}>
            <input class={classes!(field_class, "flex-grow-1")} value={amount.to_string()} onchange={update_amount} />
            <select required={true} onchange={update_type}>
                <option value={CurrencyType::Copper.to_string()} selected={true}>{CurrencyType::Copper.to_string()}</option>
                <option value={CurrencyType::Silver.to_string()}>{CurrencyType::Silver.to_string()}</option>
                <option value={CurrencyType::Gold.to_string()}>{CurrencyType::Gold.to_string()}</option>
                <option value={CurrencyType::Platinum.to_string()}>{CurrencyType::Platinum.to_string()}</option>
            </select>
        </div>
    }
}