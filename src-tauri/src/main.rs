// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, sync::{Mutex, MutexGuard}};

use common_data::{BackendError, Table, IdNamePair};
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

#[tauri::command]
fn get_tables(state: State<AppState>) -> Result<Vec<IdNamePair>, BackendError> {
    let tables = state.lock_tables()?;
    let ids = tables.iter()
        .filter_map(|(k, v)| match v.get_data() {
            Err(_) => None,
            Ok(data) => Some(IdNamePair::new(*k, data.name().to_string()))
        })
        .collect::<Vec<_>>();

    Ok(ids)
}

#[tauri::command]
fn get_table(state: State<AppState>, id: Uuid) -> Result<Table, BackendError> {
    let tables = state.lock_tables()?;
    match tables.get(&id) {
        Some(table) => Ok(table.clone()),
        None => Err(BackendError::argument_error("id", format!("Could not find table with id '{}'", id)))
    }
}

#[tauri::command]
fn new_table(state: State<AppState>, name: String) -> Result<Uuid, BackendError> {
    let mut tables = state.lock_tables()?;

    let (id, table) = Table::new(name);
    tables.insert(id, table);

    Ok(id)
}

#[tauri::command]
fn remove_table(state: State<AppState>, id: Uuid) -> Result<Table, BackendError> {
    let mut tables = state.lock_tables()?;
    tables.remove(&id).ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id)))
}

#[tauri::command]
fn insert_entry(state: State<AppState>, id: Uuid, entry: String) -> Result<(), BackendError> {
    let mut tables = state.lock_tables()?;
    let table = tables.get_mut(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id)))?;

    let mut data = table.get_data()?;
    data.push(entry);

    Ok(())
}

#[tauri::command]
fn remove_entry(state: State<AppState>, id: Uuid, index: usize) -> Result<String, BackendError> {
    let mut tables = state.lock_tables()?;
    let table = tables.get_mut(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id)))?;

    let mut data = table.get_data()?;
    
    data.remove(index)
        .ok_or(BackendError::argument_error("index", format!("Could not find entry with index '{}'", index)))
}

#[tauri::command]
fn get_random(state: State<AppState>, id: Uuid) -> Result<String, BackendError> {
    let tables = state.lock_tables()?;
    let table = tables.get(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id)))?;

    let data = table.get_data()?;
    let entry = data.get_random().map_err(|e| BackendError::from(e))?;

    Ok(entry.to_string())
}

#[tauri::command]
fn get_random_set(state: State<AppState>, id: Uuid, count: usize, allow_duplicates: bool) -> Result<Vec<String>, BackendError> {
    let tables = state.lock_tables()?;
    let table = tables.get(&id)
        .ok_or(BackendError::argument_error("id", format!("Could not find table with id '{}'", id)))?;

    let data = table.get_data()?;
    let entry = data.get_random_set(count, allow_duplicates).map_err(|e| BackendError::from(e))?;

    Ok(entry.into_iter().map(|e| e.to_string()).collect())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_tables,
            get_table,
            new_table,
            remove_table,
            insert_entry,
            remove_entry,
            get_random,
            get_random_set,
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
}
