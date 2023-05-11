use std::{path::PathBuf, cmp::Ordering, sync::{Arc, Mutex, MutexGuard}, slice::Iter, collections::HashMap};

use rand::{rngs::StdRng, SeedableRng, Rng};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{Currency, BackendError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollType {
    Cost,
    Count
}

impl RollType {
    pub fn get_values() -> Vec<Self> {
        vec! [
            Self::Cost,
            Self::Count
        ]
    }
}

impl std::fmt::Display for RollType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cost => write!(f, "Cost"),
            Self::Count => write!(f, "Count"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollLimit {
    Count(usize),
    Cost(Currency)
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
    pub fn new(cost: Currency) -> Self {
        Self {
            name: String::new(),
            cost
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

    pub fn cost_mut(&mut self) -> &mut Currency {
        &mut self.cost
    }

    pub fn set_cost(&mut self, cost: Currency) {
        self.cost = cost;
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

    pub fn total_cost(&self) -> Currency {
        self.entries.iter().map(|e| e.cost()).sum()
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

    pub fn get_random_set_by_count(&self, count: usize, allow_duplicates: bool) -> Result<Vec<RollResult>, getrandom::Error> {
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

    pub fn get_random_set_by_cost(&self, cost: Currency, allow_duplicates: bool) -> Result<Vec<RollResult>, getrandom::Error> {
        let mut remaining = cost;
        let mut rng = create_rng()?;
        let mut rolls: HashMap<usize, usize> = HashMap::new();

        while self.entries.iter().any(|entry| entry.cost() <= remaining) {
            let allowed = self.entries.iter()
                .enumerate()
                .filter_map(|(index, entry)| if entry.cost() <= remaining { Some(index) } else { None })
                .collect::<Vec<_>>();

            let mut roll = allowed[rng.gen_range(0..allowed.len())];

            if !allow_duplicates {
                while rolls.contains_key(&roll) {
                    roll = allowed[rng.gen_range(0..allowed.len())];
                }
            }

            remaining -= self.entries[roll].cost();

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