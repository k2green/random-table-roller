use std::{str::FromStr, sync::Arc};

use common_data::Currency;
use yew::prelude::*;

use crate::{components::{select::SelectDirect, number_field::NumberField}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyType {
    Platinum,
    Gold,
    Silver,
    Copper,
}

impl From<Currency> for CurrencyType {
    fn from(value: Currency) -> Self {
        match value {
            Currency::Platinum(_) => CurrencyType::Platinum,
            Currency::Gold(_) => CurrencyType::Gold,
            Currency::Silver(_) => CurrencyType::Silver,
            Currency::Copper(_) => CurrencyType::Copper,
        }
    }
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

fn get_currency(amount: u64, currency_type: CurrencyType) -> Currency {
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
    pub title: AttrValue,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub on_change: Callback<Currency>
}

#[function_component(CurrencyField)]
pub fn currency_field(props: &CurrencyFieldProps) -> Html {
    let CurrencyFieldProps { title, container_class, on_change } = props.clone();
    let amount = use_state_eq(|| 1_u64);
    let currency_type = use_state_eq(|| CurrencyType::Copper);

    html! {
        <CurrencyFieldDirect title={title} amount={amount} currency_type={currency_type} container_class={container_class} on_change={on_change} />
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CurrencyFieldDirectProps {
    pub amount: UseStateHandle<u64>,
    pub currency_type: UseStateHandle<CurrencyType>,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub on_change: Callback<Currency>
}

#[function_component(CurrencyFieldDirect)]
pub fn currency_field(props: &CurrencyFieldDirectProps) -> Html {
    let CurrencyFieldDirectProps { amount, currency_type, title, container_class, on_change } = props.clone();

    let update_amount = {
        let amount = amount.clone();
        let currency_type = currency_type.clone();
        let on_change = on_change.clone();

        Callback::from(move |val: u64| {
            amount.set(val);
            on_change.emit(get_currency(val, *currency_type));
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

    let validate = {
        let currency_type = currency_type.clone();
        Callback::from(move |amount: u64| {
            match *currency_type {
                CurrencyType::Copper => amount.clamp(1, u64::MAX),
                CurrencyType::Silver => amount.clamp(1, u64::MAX / 10),
                CurrencyType::Gold => amount.clamp(1, u64::MAX / 100),
                CurrencyType::Platinum => amount.clamp(1, u64::MAX / 1000),
            }
        })
    };

    html! {
        <div class={classes!(container_class, "flex-row")}>
            <NumberField<u64> title={title} class="number" value={*amount} get_default={|_: ()| 1_u64} validate={validate} on_change={update_amount} />
            <SelectDirect<CurrencyType> items={Arc::new(CurrencyType::get_all())} selected_item={currency_type} on_change={update_type} />
        </div>
    }
}