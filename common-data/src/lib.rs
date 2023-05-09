use std::{sync::{Mutex, Arc, MutexGuard}, slice::Iter, collections::HashMap};

use rand::{SeedableRng, rngs::StdRng, Rng};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

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
    pub fn new(name: impl Into<String>, order: usize) -> (Uuid, Self) {
        let table = TableData::new(name, order);
        let id = table.id();

        (id, Self { data: Arc::new(Mutex::new(table)) })
    }

    pub fn with_capacity(name: impl Into<String>, capacity: usize, order: usize) -> (Uuid, Self) {
        let table = TableData::with_capacity(name, capacity, order);
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
    entry: String
}

impl RollResult {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn entry(&self) -> &str {
        &self.entry
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableData {
    id: Uuid,
    order: usize,
    name: String,
    entries: Vec<String>
}

impl TableData {
    pub fn new(name: impl Into<String>, order: usize) -> TableData {
        Self {
            order,
            id: Uuid::new_v4(),
            name: name.into(),
            entries: Vec::new(),
        }
    }

    pub fn with_capacity(name: impl Into<String>, capacity: usize, order: usize) -> TableData {
        Self {
            order,
            id: Uuid::new_v4(),
            name: name.into(),
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn sort(&mut self) {
        self.entries.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
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

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn iter(&self) -> Iter<String> {
        self.entries.iter()
    }

    pub fn push(&mut self, item: impl Into<String>) {
        self.entries.push(item.into());
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.entries.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut String> {
        self.entries.get_mut(index)
    }

    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index >= self.len() {
            None
        } else {
            Some(self.entries.remove(index))
        }
    }

    pub fn get_random(&self) -> Result<&str, getrandom::Error> {
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

        output.sort_by(|a, b| a.entry().to_lowercase().cmp(&b.entry().to_lowercase()));

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