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