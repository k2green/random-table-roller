use std::{sync::{Mutex, Arc, MutexGuard}, slice::Iter, collections::HashMap, path::PathBuf, cmp::Ordering};

use rand::{SeedableRng, rngs::StdRng, Rng};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdNamePair {
    id: Uuid,
    name: String
}

impl IdNamePair {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    data: Arc<Mutex<TableData>>
}

impl From<TableData> for Table {
    fn from(value: TableData) -> Self {
        Self {
            data: Arc::new(Mutex::new(value))
        }
    }
}

impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.data.lock().unwrap().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let data = TableData::deserialize(deserializer)?;
        Ok(Self { data: Arc::new(Mutex::new(data)) })
    }
}

impl Table {
    pub fn new(use_cost: bool, name: impl Into<String>, order: usize) -> (Uuid, Self) {
        let table = TableData::new(use_cost, name, order);
        let id = table.id();

        (id, Self { data: Arc::new(Mutex::new(table)) })
    }

    pub fn with_capacity(use_cost: bool, name: impl Into<String>, capacity: usize, order: usize) -> (Uuid, Self) {
        let table = TableData::with_capacity(use_cost, name, capacity, order);
        let id = table.id();

        (id, Self { data: Arc::new(Mutex::new(table)) })
    }

    pub fn get_data(&self) -> Result<MutexGuard<TableData>, BackendError> {
        self.data.lock().map_err(|_| BackendError::internal_error("Unable to lock table data"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollResult {
    count: usize,
    entry: TableEntry
}

impl RollResult {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn entry(&self) -> &TableEntry {
        &self.entry
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileTableData {
    use_cost: bool,
    name: String,
    entries: Vec<TableEntry>,
}

impl FileTableData {
    pub fn into_table_data(self, order: usize, path: Option<PathBuf>) -> TableData {
        TableData {
            use_cost: self.use_cost,
            id: Uuid::new_v4(),
            order,
            name: self.name,
            entries: self.entries,
            path
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableEntry {
    name: String,
    cost: Currency
}

impl PartialOrd for TableEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.to_lowercase().partial_cmp(&other.name.to_lowercase())
    }
}

impl Ord for TableEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl TableEntry {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            cost: Currency::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn cost(&self) -> Currency {
        self.cost
    }

    pub fn set_cost(&mut self, cost: Currency) {
        self.cost = cost;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableData {
    use_cost: bool,
    id: Uuid,
    #[serde(skip)]
    order: usize,
    name: String,
    entries: Vec<TableEntry>,
    path: Option<PathBuf>
}

impl TableData {
    pub fn new(use_cost: bool, name: impl Into<String>, order: usize) -> TableData {
        Self {
            use_cost,
            order,
            id: Uuid::new_v4(),
            name: name.into(),
            entries: Vec::new(),
            path: None
        }
    }

    pub fn with_capacity(use_cost: bool, name: impl Into<String>, capacity: usize, order: usize) -> TableData {
        Self {
            use_cost,
            order,
            id: Uuid::new_v4(),
            name: name.into(),
            entries: Vec::with_capacity(capacity),
            path: None
        }
    }

    pub fn to_file_data(&self) -> FileTableData {
        FileTableData {
            use_cost: self.use_cost,
            name: self.name.clone(),
            entries: self.entries.clone()
        }
    }

    pub fn sort(&mut self) {
        self.entries.sort();
    }

    pub fn use_cost(&self) -> bool {
        self.use_cost
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn set_order(&mut self, order: usize) {
        self.order = order;
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.path.clone()
    }

    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path;
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn iter(&self) -> Iter<TableEntry> {
        self.entries.iter()
    }

    pub fn push(&mut self, item: impl Into<TableEntry>) {
        self.entries.push(item.into());
    }

    pub fn get(&self, index: usize) -> Option<&TableEntry> {
        self.entries.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut TableEntry> {
        self.entries.get_mut(index)
    }

    pub fn remove(&mut self, index: usize) -> Option<TableEntry> {
        if index >= self.len() {
            None
        } else {
            Some(self.entries.remove(index))
        }
    }

    pub fn get_random(&self) -> Result<&TableEntry, getrandom::Error> {
        let mut rng = create_rng()?;
        Ok(&self.entries[rng.gen_range(0..self.len())])
    }

    pub fn get_random_set(&self, count: usize, allow_duplicates: bool) -> Result<Vec<RollResult>, getrandom::Error> {
        let mut rng = create_rng()?;
        let mut rolls: HashMap<usize, usize> = HashMap::new();

        for _ in 0..count {
            let mut roll = rng.gen_range(0..self.len());

            if !allow_duplicates {
                while rolls.contains_key(&roll) {
                    roll = rng.gen_range(0..self.len());
                }
            }

            match rolls.get_mut(&roll) {
                Some(rolls) => *rolls += 1,
                None => { rolls.insert(roll, 1); }
            };
        }

        let mut output = rolls.into_iter()
            .map(|(roll, count)| RollResult {
                count,
                entry: self.entries[roll].clone()
            })
            .collect::<Vec<_>>();

        output.sort_by(|a, b| a.entry().cmp(&b.entry()));

        Ok(output)
    }
}

pub fn create_rng() -> Result<StdRng, getrandom::Error> {
    let mut buffer = [0_u8; 32];
    getrandom::getrandom(&mut buffer)?;

    Ok(StdRng::from_seed(buffer))
}

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