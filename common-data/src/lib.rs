pub mod currency;
pub mod id_name_pair;
pub mod table;

use serde::{Serialize, Deserialize};

pub use crate::currency::Currency;
pub use crate::id_name_pair::IdNamePair;
pub use crate::table::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum BackendError {
    InternalError(String),
    ArgumentError{ arg_name: String, message: String }
}

impl<E: std::error::Error> From<E> for BackendError {
    fn from(value: E) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
            Self::ArgumentError { arg_name, message } => write!(f, "Error with argument '{}': {}", arg_name, message)
        }
    }
}

impl BackendError {
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError(message.into())
    }

    pub fn argument_error(arg_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ArgumentError {
            arg_name: arg_name.into(),
            message: message.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn currency_serialization_test_base(currency: Currency) {
        let serialized = serde_json::to_string(&currency).unwrap();
        let deserialized: Currency = serde_json::from_str(&serialized).unwrap();

        println!("Original: {:?}\nSerialized: {}\n", &currency, serialized);
        assert_eq!(currency, deserialized);
    }

    #[test]
    fn currency_serialization() {
        currency_serialization_test_base(Currency::Copper(1));
        currency_serialization_test_base(Currency::Silver(1));
        currency_serialization_test_base(Currency::Gold(1));
        currency_serialization_test_base(Currency::Platinum(1));
    }
}