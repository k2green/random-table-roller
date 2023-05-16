use std::{str::FromStr, fmt::Display};

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::try_parse;

pub trait Number: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Display + FromStr + Default + 'static {
    fn get_max() -> Self;
    fn get_min() -> Self;
}

fn validate_callback<T: TargetCast, N: Number>(
    value: N,
    validate: Callback<N, N>,
    get_default: Option<Callback<(), N>>,
    on_change: Option<Callback<N>>
) -> Callback<T> {
    Callback::from(move |e: T| {
        let get_default = get_default.clone();
        let on_change = on_change.clone();
        let target: HtmlInputElement = e.target_unchecked_into();
        let str_value = target.value();

        let new_value = if str_value.trim().is_empty() {
            match get_default {
                Some(get_default) => get_default.emit(()),
                None => N::default()
            }
        } else {
            match try_parse(&str_value, N::from_str) {
                Some(value) => validate.emit(value),
                None => value
            }
        };
        
        let clamped = new_value.clamp(N::get_min(), N::get_max());

        target.set_value(&clamped.to_string());
        
        if let Some(on_change) = on_change {
            on_change.emit(new_value);
        }
    })
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NumberFieldProps<T: Number> {
    pub value: T,
    pub validate: Callback<T, T>,
    #[prop_or_default]
    pub get_default: Option<Callback<(), T>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_change: Callback<T>
}

#[function_component(NumberField)]
pub fn number_field<T: Number>(props: &NumberFieldProps<T>) -> Html {
    let NumberFieldProps { value, validate, get_default, class, on_change } = props.clone();
    
    let on_input = validate_callback::<InputEvent, _>(value, validate.clone(), get_default.clone(), None);
    let on_change = validate_callback::<Event, _>(value, validate.clone(), get_default.clone(), Some(on_change));

    html! {
        <input class={class} value={value.to_string()} oninput={on_input} onchange={on_change} />
    }
}

impl Number for usize {
    fn get_max() -> usize { usize::MAX }
    fn get_min() -> usize { usize::MIN }
}

impl Number for u8 {
    fn get_max() -> u8 { u8::MAX }
    fn get_min() -> u8 { u8::MIN }
}

impl Number for u16 {
    fn get_max() -> u16 { u16::MAX }
    fn get_min() -> u16 { u16::MIN }
}

impl Number for u32 {
    fn get_max() -> u32 { u32::MAX }
    fn get_min() -> u32 { u32::MIN }
}

impl Number for u64 {
    fn get_max() -> u64 { u64::MAX }
    fn get_min() -> u64 { u64::MIN }
}

impl Number for u128 {
    fn get_max() -> u128 { u128::MAX }
    fn get_min() -> u128 { u128::MIN }
}

impl Number for isize {
    fn get_max() -> isize { isize::MAX }
    fn get_min() -> isize { isize::MIN }
}

impl Number for i8 {
    fn get_max() -> i8 { i8::MAX }
    fn get_min() -> i8 { i8::MIN }
}

impl Number for i16 {
    fn get_max() -> i16 { i16::MAX }
    fn get_min() -> i16 { i16::MIN }
}

impl Number for i32 {
    fn get_max() -> i32 { i32::MAX }
    fn get_min() -> i32 { i32::MIN }
}

impl Number for i64 {
    fn get_max() -> i64 { i64::MAX }
    fn get_min() -> i64 { i64::MIN }
}

impl Number for i128 {
    fn get_max() -> i128 { i128::MAX }
    fn get_min() -> i128 { i128::MIN }
}