use std::{cmp::Ordering, ops::{Add, Sub, AddAssign, SubAssign}, iter::Sum, str::FromStr};

use base64::{engine::general_purpose, Engine};
use byteorder::ReadBytesExt;
use regex::Regex;
use serde::{Serialize, Deserialize, de::{Visitor, Error}};

const BASE_64_PATTERN: &'static str = r"^([A-Za-z0-9+/]{4})*[A-Za-z0-9+/]{2}==|([A-Za-z0-9+/]{4})*[A-Za-z0-9+/]{3}=|([A-Za-z0-9+/]{4})*[A-Za-z0-9+/]{4}$";
const FORMMATED_CURRENCY_PATTERN: &'static str = r"^\s*(\d+)\s+([csgpCSGP][pP])\s*$";

#[derive(Debug, Clone)]
pub enum ParseCurrencyError {
    RegexError(regex::Error),
    ParseIntError(std::num::ParseIntError),
    Custom(String)
}

impl std::fmt::Display for ParseCurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegexError(e) => write!(f, "Regex error: {}", e),
            Self::ParseIntError(e) => write!(f, "Parse error: {}", e),
            Self::Custom(message) => write!(f, "{}", message)
        }
    }
}

impl std::error::Error for ParseCurrencyError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::RegexError(e) => Some(e),
            Self::ParseIntError(e) => Some(e),
            Self::Custom(_) => None
        }
    }
}

impl ParseCurrencyError {
    fn custom(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }
}

pub enum CurrencyVisitorResult {
    Base64(String),
    Formatted(String),
    Integer(u64)
}

struct CurrencyVisitor;

impl<'de> Visitor<'de> for CurrencyVisitor {
    type Value = CurrencyVisitorResult;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a base64 string, a formatted price or an integer")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        get_result_from_str(&v).ok_or(E::custom("String was not a base64 or formatted string"))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        get_result_from_str(v).ok_or(E::custom("String was not a base64 or formatted string"))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(CurrencyVisitorResult::Integer(v))
    }
}

fn matches(pattern: &str, text: &str) -> bool {
    match Regex::new(pattern) {
        Ok(pattern) => pattern.is_match(text),
        Err(_) => false,
    }
}

fn get_result_from_str(text: &str) -> Option<CurrencyVisitorResult> {
    if matches(BASE_64_PATTERN, text) {
        Some(CurrencyVisitorResult::Base64(text.to_string()))
    } else if matches(FORMMATED_CURRENCY_PATTERN, text) {
        Some(CurrencyVisitorResult::Formatted(text.trim().to_string()))
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Platinum(u64),
    Gold(u64),
    Silver(u64),
    Copper(u64)
}

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(FORMMATED_CURRENCY_PATTERN).map_err(ParseCurrencyError::RegexError)?;
        let captures = pattern.captures(s).ok_or(ParseCurrencyError::custom("Could not get capture groups"))?;
        let amount_capture = captures.get(1).ok_or(ParseCurrencyError::custom("Could not get capture group 1"))?.as_str();
        let currency_capture = captures.get(2).ok_or(ParseCurrencyError::custom("Could not get capture group 2"))?.as_str();

        let amount = u64::from_str_radix(amount_capture, 10).map_err(ParseCurrencyError::ParseIntError)?;

        match currency_capture.to_lowercase().as_str() {
            "cp" => Ok(Self::Copper(amount)),
            "sp" => Ok(Self::Silver(amount)),
            "gp" => Ok(Self::Gold(amount)),
            "pp" => Ok(Self::Platinum(amount)),
            _ => Err(ParseCurrencyError::custom(format!("Unexpected currency type: {}", currency_capture)))
        }
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_u64(self.to_copper().amount())
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let data = deserializer.deserialize_any(CurrencyVisitor)?;
        let output = match data {
            CurrencyVisitorResult::Base64(base64) => get_currency_from_base64(&base64),
            CurrencyVisitorResult::Formatted(formatted) => get_currency_from_formatted(&formatted),
            CurrencyVisitorResult::Integer(value) => Ok(Currency::from(value))
        };

        output.map_err(|msg| Error::custom(msg))
    }
}

fn get_currency_from_base64(base64: &str) -> Result<Currency, String> {
    let bytes = general_purpose::STANDARD.decode(base64).expect("Could not decode string");
    let mut reader = bytes.as_slice();

    let value = if bytes.len() > 8 {
        reader.read_uint128::<byteorder::BigEndian>(16)
            .map(|v| (v & 0xffffffffffffffff) as u64)
            .map_err(|e| e.to_string())?
    } else {
        reader.read_u64::<byteorder::BigEndian>()
            .map_err(|e| e.to_string())?
    };

    Ok(Currency::from(value))
}

fn get_currency_from_formatted(fromatted: &str) -> Result<Currency, String> {
    Currency::from_str(fromatted).map_err(|e| e.to_string())
}

impl Default for Currency {
    fn default() -> Self {
        Self::Copper(0)
    }
}

impl From<u64> for Currency {
    fn from(value: u64) -> Self {
        Self::Copper(value).to_largest_denomination()
    }
}

impl From<Currency> for u64 {
    fn from(value: Currency) -> Self {
        value.to_copper().amount()
    }
}

impl Add<Currency> for Currency {
    type Output = Currency;

    fn add(self, rhs: Currency) -> Self::Output {
        let a_amount = self.to_copper().amount();
        let b_amount = rhs.to_copper().amount();

        Currency::from(logged_add(a_amount, b_amount))
    }
}

impl AddAssign<Currency> for Currency {
    fn add_assign(&mut self, rhs: Currency) {
        *self = *self + rhs;
    }
}

impl Sub<Currency> for Currency {
    type Output = Currency;

    fn sub(self, rhs: Currency) -> Self::Output {
        let a_amount = self.to_copper().amount();
        let b_amount = rhs.to_copper().amount();

        Currency::from(logged_sub(a_amount, b_amount))
    }
}

impl SubAssign<Currency> for Currency {
    fn sub_assign(&mut self, rhs: Currency) {
        *self = *self - rhs;
    }
}

impl Sum for Currency {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut acc = 0_u64;

        for item in iter {
            let amount = item.to_copper().amount();
            acc = logged_add(acc, amount);
        }

        Self::from(acc)
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_amount = self.to_copper().amount();
        let other_amount = other.to_copper().amount();

        self_amount.partial_cmp(&other_amount)
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_amount = self.to_copper().amount();
        let other_amount = other.to_copper().amount();

        self_amount.cmp(&other_amount)
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Platinum(amount) => write!(f, "{} pp", amount),
            Self::Gold(amount) => write!(f, "{} gp", amount),
            Self::Silver(amount) => write!(f, "{} sp", amount),
            Self::Copper(amount) => write!(f, "{} cp", amount),
        }
    }
}

impl Currency {
    pub fn with_amount(self, amount: u64) -> Self {
        match self {
            Self::Platinum(_) => Self::Platinum(amount),
            Self::Gold(_) => Self::Gold(amount),
            Self::Silver(_) => Self::Silver(amount),
            Self::Copper(_) => Self::Copper(amount),
        }
    }

    pub fn to_largest_denomination(self) -> Self {
        let mut current = self;

        while let Some(new) = current.try_convert_up() {
            current = new;
        }

        current
    }

    pub fn try_convert_up(&self) -> Option<Self>{
        let amount = self.amount();
        if amount > 0 && amount % 10 == 0 {
            match self {
                Self::Platinum(_) => None,
                Self::Gold(amount) => Some(Self::Platinum(logged_div(*amount, 10))),
                Self::Silver(amount) => Some(Self::Gold(logged_div(*amount, 10))),
                Self::Copper(amount) => Some(Self::Silver(logged_div(*amount, 10))),
            }
        } else {
            None
        }
    }

    pub fn convert_down(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Gold(logged_mul(amount, 10)),
            Self::Gold(amount) => Self::Silver(logged_mul(amount, 10)),
            Self::Silver(amount) => Self::Copper(logged_mul(amount, 10)),
            Self::Copper(_) => self,
        }
    }

    pub fn amount(&self) -> u64 {
        match self {
            Self::Platinum(amount) => *amount,
            Self::Gold(amount) => *amount,
            Self::Silver(amount) => *amount,
            Self::Copper(amount) => *amount,
        }
    }

    pub fn to_platinum(self) -> Self {
        match self {
            Self::Platinum(_) => self,
            Self::Gold(amount) => Self::Platinum(logged_div(amount, 10)),
            Self::Silver(amount) => Self::Platinum(logged_div(amount, 100)),
            Self::Copper(amount) => Self::Platinum(logged_div(amount, 1000)),
        }
    }

    pub fn to_gold(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Gold(logged_mul(amount, 10)),
            Self::Gold(_) => self,
            Self::Silver(amount) => Self::Gold(logged_div(amount, 10)),
            Self::Copper(amount) => Self::Gold(logged_div(amount, 100)),
        }
    }

    pub fn to_silver(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Silver(logged_mul(amount, 100)),
            Self::Gold(amount) => Self::Silver(logged_mul(amount, 10)),
            Self::Silver(_) => self,
            Self::Copper(amount) => Self::Silver(logged_div(amount, 10)),
        }
    }

    pub fn to_copper(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Copper(logged_mul(amount, 1000)),
            Self::Gold(amount) => Self::Copper(logged_mul(amount, 100)),
            Self::Silver(amount) => Self::Copper(logged_mul(amount, 10)),
            Self::Copper(_) => self,
        }
    }
}

fn logged_add(a: u64, b: u64) -> u64 {
    match a.checked_add(b) {
        Some(res) => res,
        None => {
            log::error!("Overflow occurred when adding '{}' to '{}'", b, a);
            panic!("Overflow");
        }
    }
}

fn logged_sub(a: u64, b: u64) -> u64 {
    match a.checked_sub(b) {
        Some(res) => res,
        None => {
            log::error!("Overflow occurred when subtracting '{}' from '{}'", b, a);
            panic!("Overflow");
        }
    }
}

fn logged_mul(a: u64, b: u64) -> u64 {
    match a.checked_mul(b) {
        Some(res) => res,
        None => {
            log::error!("Overflow occurred when multiplying '{}' by '{}'", a, b);
            panic!("Overflow");
        }
    }
}

fn logged_div(a: u64, b: u64) -> u64 {
    match a.checked_div(b) {
        Some(res) => res,
        None => {
            log::error!("Overflow occurred when multiplying '{}' by '{}'", a, b);
            panic!("Overflow");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_currency_serialization_test_base(currency: Currency) {
        let serialized = serde_json::to_string(&currency).unwrap();
        let deserialized: Currency = serde_json::from_str(&serialized).unwrap();

        println!("Value {}, Serialized: {}", currency, serialized);
        assert_eq!(currency, deserialized);
    }
    
    #[test]
    fn currency_convert_up() {
        assert_eq!(Currency::Silver(1), Currency::Copper(10).to_silver());

        assert_eq!(Currency::Gold(1), Currency::Copper(100).to_gold());
        assert_eq!(Currency::Gold(1), Currency::Silver(10).to_gold());

        assert_eq!(Currency::Platinum(1), Currency::Copper(1000).to_platinum());
        assert_eq!(Currency::Platinum(1), Currency::Silver(100).to_platinum());
        assert_eq!(Currency::Platinum(1), Currency::Gold(10).to_platinum());
    }

    #[test]
    fn currency_convert_down() {
        assert_eq!(Currency::Copper(10), Currency::Silver(1).to_copper());
        assert_eq!(Currency::Copper(100), Currency::Gold(1).to_copper());
        assert_eq!(Currency::Copper(1000), Currency::Platinum(1).to_copper());

        assert_eq!(Currency::Silver(10), Currency::Gold(1).to_silver());
        assert_eq!(Currency::Silver(100), Currency::Platinum(1).to_silver());
        
        assert_eq!(Currency::Gold(10), Currency::Platinum(1).to_gold());
    }

    #[test]
    fn currency_to_largest() {
        assert_eq!(Currency::Silver(1), Currency::Copper(10).to_largest_denomination());
        assert_eq!(Currency::Gold(1), Currency::Copper(100).to_largest_denomination());
        assert_eq!(Currency::Platinum(1), Currency::Copper(1000).to_largest_denomination());
        assert_eq!(Currency::Copper(1), Currency::Copper(1).to_largest_denomination());
        assert_eq!(Currency::Copper(19), Currency::Copper(19).to_largest_denomination());

        assert_eq!(Currency::Gold(1), Currency::Silver(10).to_largest_denomination());
        assert_eq!(Currency::Platinum(1), Currency::Silver(100).to_largest_denomination());
        assert_eq!(Currency::Silver(1), Currency::Silver(1).to_largest_denomination());
        assert_eq!(Currency::Silver(19), Currency::Silver(19).to_largest_denomination());

        assert_eq!(Currency::Platinum(1), Currency::Gold(10).to_largest_denomination());
        assert_eq!(Currency::Gold(1), Currency::Gold(1).to_largest_denomination());
        assert_eq!(Currency::Gold(19), Currency::Gold(19).to_largest_denomination());
    }

    #[test]
    fn test_currency_serialization() {
        test_currency_serialization_test_base(Currency::Copper(1));
        test_currency_serialization_test_base(Currency::Copper(123));
        test_currency_serialization_test_base(Currency::Silver(1));
        test_currency_serialization_test_base(Currency::Silver(123));
        test_currency_serialization_test_base(Currency::Gold(1));
        test_currency_serialization_test_base(Currency::Gold(123));
        test_currency_serialization_test_base(Currency::Platinum(1));
        test_currency_serialization_test_base(Currency::Platinum(123));
    }
}