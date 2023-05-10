use std::{str::FromStr, sync::Arc};

use common_data::Currency;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{try_parse_number, components::select::Select};

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

impl CurrencyType {
    fn get_all() -> Vec<Self> {
        vec! [
            Self::Copper,
            Self::Silver,
            Self::Gold,
            Self::Platinum
        ]
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
    pub container_class: Classes,
    #[prop_or_default]
    pub field_class: Classes,
    pub on_change: Callback<Currency>
}

#[function_component(CurrencyField)]
pub fn currency_field(props: &CurrencyFieldProps) -> Html {
    let CurrencyFieldProps { container_class, field_class, on_change } = props.clone();
    let amount = use_state_eq(|| 1_usize);
    let currency_type = use_state_eq(|| CurrencyType::Copper);

    let update_amount = {
        let amount = amount.clone();
        let currency_type = currency_type.clone();
        let on_change = on_change.clone();

        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let target_value = target.value();
            if let Some(parsed) = try_parse_number(&target_value) {
                let parsed = parsed.max(1);
                amount.set(parsed);
                on_change.emit(get_currency(parsed, *currency_type));
            }
        })
    };

    let update_type = {
        let amount = amount.clone();
        let currency_type = currency_type.clone();
        let on_change = on_change.clone();

        Callback::from(move |new: CurrencyType| {
            currency_type.set(new);
            on_change.emit(get_currency(*amount, new));
        })
    };

    html! {
        <div class={classes!(container_class, "flex-row")}>
            <input type="number" class={classes!(field_class, "flex-grow-1", "hor-margin")} min="1" value={amount.to_string()} onchange={update_amount} />
            <Select<CurrencyType> items={Arc::new(CurrencyType::get_all())} on_change={update_type} />
        </div>
    }
}