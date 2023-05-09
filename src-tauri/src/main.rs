// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod logging;

use std::{collections::HashMap, sync::{Mutex, MutexGuard}, cmp::Ordering, path::PathBuf, fs::{OpenOptions, self, File}};

use common_data::{BackendError, Table, IdNamePair, TableData, RollResult};
use log::SetLoggerError;
use logging::{setup_logging, cleanup_logs};
use tauri::{State, Manager};
use uuid::Uuid;

struct AppState {
    tables: Mutex<HashMap<Uuid, Table>>
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tables: Mutex::new(HashMap::new())
        }
    }
}

impl AppState {
    fn lock_tables(&self) -> Result<MutexGuard<HashMap<Uuid, Table>>, BackendError> {
        self.tables.lock().map_err(|_| BackendError::internal_error("Unable to lock tables"))
    }
}

fn log_result<T, E: std::fmt::Display>(result: Result<T, E>) -> Result<T, E> {
    if let Err(e) = &result {
        log::error!("{}", e);
    }

    result
}

#[tauri::command]
fn get_tables(state: State<AppState>) -> Result<Vec<IdNamePair>, BackendError> {
    log::info!("Getting tables...");
    let tables = log_result(state.lock_tables())?;
    let mut table_vec = tables.iter().collect::<Vec<_>>();

    table_vec.sort_by(|(_, a), (_, b)| table_order(a, b));

    let ids = table_vec.into_iter()
        .filter_map(|(k, v)| match v.get_data() {
            Err(_) => None,
            Ok(data) => Some(IdNamePair::new(*k, data.name().to_string()))
        })
        .collect::<Vec<_>>();

    Ok(ids)
}

fn table_order(a: &Table, b: &Table) -> Ordering {
    let a_guard = match a.get_data() {
        Ok(guard) => guard,
        Err(_) => return Ordering::Equal
    };

    let b_guard = match b.get_data() {
        Ok(guard) => guard,
        Err(_) => return Ordering::Equal
    };

    a_guard.order().cmp(&b_guard.order())
}

#[tauri::command]
fn get_table(state: State<AppState>, id: Uuid) -> Result<Table, BackendError> {
    log::info!("Getting table with id '{}'...", id);
    let tables = log_result(state.lock_tables())?;
    log_result(match tables.get(&id) {
        Some(table) => Ok(table.clone()),
        None => Err(BackendError::argument_error("id", format!("Could not find table with id '{}'", id)))
    })
}

#[tauri::command]
fn new_table(state: State<AppState>, name: String, entries: String) -> Result<Uuid, BackendError> {
    log::info!("Adding new table with name '{}'...", &name);
    let mut tables = log_result(state.lock_tables())?;

    let mut table_data = TableData::new(name, tables.len());
    let id = table_data.id();

    for line in entries.lines() {
        let trimmed = line.trim();

        if !trimmed.is_empty() {
            table_data.push(trimmed);
        }
    }

    table_data.sort();
    tables.insert(id, Table::from(table_data));

    Ok(id)
}

#[tauri::command]
fn remove_table(state: State<AppState>, id: Uuid) -> Result<Table, BackendError> {
    log::info!("Removing table with id '{}'...", id);
    let mut tables = log_result(state.lock_tables())?;
    log_result(tables.remove(&id).ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id))))
}

#[tauri::command]
fn change_table_name(state: State<AppState>, id: Uuid, name: String) -> Result<(), BackendError> {
    let mut tables = log_result(state.lock_tables())?;
    let table = log_result(tables.get_mut(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id))))?;

    let mut data = log_result(table.get_data().map_err(BackendError::from))?;
    data.set_name(name);

    Ok(())
}

#[tauri::command]
fn add_entries(state: State<AppState>, id: Uuid, entries: String) -> Result<(), BackendError> {
    log::info!("Adding '{:?}' to table with id '{}'...", &entries, id);
    let mut tables = log_result(state.lock_tables())?;
    let table = log_result(tables.get_mut(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id))))?;

    let mut data = table.get_data()?;
    for line in entries.lines() {
        let trimmed = line.trim();

        if !trimmed.is_empty() {
            data.push(line);
        }
    }

    data.sort();

    Ok(())
}

#[tauri::command]
fn remove_entry(state: State<AppState>, id: Uuid, index: usize) -> Result<String, BackendError> {
    log::info!("Removing entry {} from table with id '{}'...", index, id);
    let mut tables = log_result(state.lock_tables())?;
    let table = log_result(tables.get_mut(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id))))?;

    let mut data = table.get_data()?;
    
    log_result(data.remove(index)
        .ok_or(BackendError::argument_error("index", format!("Could not find entry with index '{}'", index))))
}

#[tauri::command]
fn get_random(state: State<AppState>, id: Uuid) -> Result<String, BackendError> {
    log::info!("Getting random entry from table with id '{}'...", id);
    let tables = log_result(state.lock_tables())?;
    let table = log_result(tables.get(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id))))?;

    let data = table.get_data()?;
    let entry = log_result(data.get_random().map_err(|e| BackendError::from(e)))?;

    Ok(entry.to_string())
}

#[tauri::command]
fn get_random_set(state: State<AppState>, id: Uuid, count: usize, allow_duplicates: bool) -> Result<Vec<RollResult>, BackendError> {
    log::info!("Getting {} random entries from table with id '{}'...", count, id);
    let tables = log_result(state.lock_tables())?;
    let table = log_result(tables.get(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id))))?;

    let data = table.get_data()?;
    let entry = log_result(data.get_random_set(count, allow_duplicates).map_err(|e| BackendError::from(e)))?;

    log::info!("Random rolls: {:?}", &entry);

    Ok(entry)
}

#[tauri::command]
fn save_table(state: State<AppState>, id: Uuid, path: PathBuf) -> Result<(), BackendError> {
    let tables = log_result(state.lock_tables())?;
    let table = log_result(tables.get(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id))))?;

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            log_result(fs::create_dir_all(parent).map_err(BackendError::from))?;
        }
    }

    let file = log_result(File::create(path).map_err(BackendError::from))?;
    log_result(serde_json::to_writer_pretty(file, table).map_err(BackendError::from))?;

    Ok(())
}

#[tauri::command]
fn open_table(state: State<AppState>, path: PathBuf) -> Result<(), BackendError> {
    let mut tables = log_result(state.lock_tables())?;
    let file = log_result(File::open(path).map_err(BackendError::from))?;
    let mut table_data: TableData = log_result(serde_json::from_reader(file).map_err(BackendError::from))?;
    let id = table_data.id();

    table_data.set_order(tables.len());
    tables.insert(id, Table::from(table_data));

    Ok(())
}

fn get_test_state() -> AppState {
    let mut tables = HashMap::new();

    for i in 0..5 {
        let mut table = TableData::new(format!("Test table {}", i), tables.len());
        let id = table.id();

        for j in 0..10 {
            table.push(format!("Table {} entry {}", i, j));
        }

        tables.insert(id, Table::from(table));
    }

    AppState { tables: Mutex::new(tables) }
}

fn main() -> Result<(), SetLoggerError> {
    cleanup_logs().ok();
    if let Err(e) = setup_logging() {
        log::error!("{}", e);
    }

    log::info!("Starting backend...");

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_tables,
            get_table,
            new_table,
            remove_table,
            change_table_name,
            add_entries,
            remove_entry,
            get_random,
            get_random_set,
            save_table,
            open_table,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
