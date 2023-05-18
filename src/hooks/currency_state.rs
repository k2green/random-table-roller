use common_data::Currency;
use yew::prelude::*;

use crate::components::currency_field::CurrencyType;

#[derive(Debug, Clone, PartialEq)]
pub struct UseCurrencyStateHandle {
    amount: UseStateHandle<u64>,
    currency_type: UseStateHandle<CurrencyType>
}

impl UseCurrencyStateHandle {
    pub fn set(&self, currency: Currency) {
        self.set_amount(currency.amount());
        self.set_currency_type(CurrencyType::from(currency));
    }

    pub fn currency(&self) -> Currency {
        match self.currency_type() {
            CurrencyType::Platinum => Currency::Platinum(self.amount()),
            CurrencyType::Gold => Currency::Gold(self.amount()),
            CurrencyType::Silver => Currency::Silver(self.amount()),
            CurrencyType::Copper => Currency::Copper(self.amount()),
        }
    }

    pub fn amount(&self) -> u64 {
        *self.amount
    }

    pub fn amount_handle(&self) -> UseStateHandle<u64> {
        self.amount.clone()
    }

    pub fn set_amount(&self, value: u64) {
        self.amount.set(value)
    }

    pub fn currency_type(&self) -> CurrencyType {
        *self.currency_type
    }

    pub fn currency_type_handle(&self) -> UseStateHandle<CurrencyType> {
        self.currency_type.clone()
    }

    pub fn set_currency_type(&self, value: CurrencyType) {
        self.currency_type.set(value);
    }
}

#[hook]
pub fn use_currency_state<F: Fn() -> Currency>(init: F) -> UseCurrencyStateHandle {
    let amount = use_state(|| init().amount());
    let currency_type = use_state(|| CurrencyType::from(init()));

    UseCurrencyStateHandle { amount, currency_type }
}

#[hook]
pub fn use_currency_state_eq<F: Fn() -> Currency>(init: F) -> UseCurrencyStateHandle {
    let amount = use_state_eq(|| init().amount());
    let currency_type = use_state_eq(|| CurrencyType::from(init()));

    UseCurrencyStateHandle { amount, currency_type }
}