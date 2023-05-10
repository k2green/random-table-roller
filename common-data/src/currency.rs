use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Platinum(usize),
    Gold(usize),
    Silver(usize),
    Copper(usize)
}

impl Default for Currency {
    fn default() -> Self {
        Self::Copper(0)
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.to_copper().amount().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let amount = usize::deserialize(deserializer)?;
        Ok(Currency::Copper(amount).to_largest_denomination())
    }
}

impl From<usize> for Currency {
    fn from(value: usize) -> Self {
        Self::Copper(value).to_largest_denomination()
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
                Self::Gold(amount) => Some(Self::Platinum(amount / 10)),
                Self::Silver(amount) => Some(Self::Gold(amount / 10)),
                Self::Copper(amount) => Some(Self::Silver(amount / 10)),
            }
        } else {
            None
        }
    }

    pub fn convert_down(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Gold(amount * 10),
            Self::Gold(amount) => Self::Silver(amount * 10),
            Self::Silver(amount) => Self::Copper(amount * 10),
            Self::Copper(_) => self,
        }
    }

    pub fn amount(&self) -> usize {
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
            Self::Gold(amount) => Self::Platinum(amount / 10),
            Self::Silver(amount) => Self::Platinum(amount / 100),
            Self::Copper(amount) => Self::Platinum(amount / 1000),
        }
    }

    pub fn to_gold(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Gold(amount * 10),
            Self::Gold(_) => self,
            Self::Silver(amount) => Self::Gold(amount / 10),
            Self::Copper(amount) => Self::Gold(amount / 100),
        }
    }

    pub fn to_silver(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Silver(amount * 100),
            Self::Gold(amount) => Self::Silver(amount * 10),
            Self::Silver(_) => self,
            Self::Copper(amount) => Self::Silver(amount / 10),
        }
    }

    pub fn to_copper(self) -> Self {
        match self {
            Self::Platinum(amount) => Self::Copper(amount * 1000),
            Self::Gold(amount) => Self::Copper(amount * 100),
            Self::Silver(amount) => Self::Copper(amount * 10),
            Self::Copper(_) => self,
        }
    }
}