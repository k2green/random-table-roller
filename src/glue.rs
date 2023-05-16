use std::path::PathBuf;

use common_data::{IdNamePair, TableData, RollResult, TableEntry, RollLimit};
use serde::Serialize;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::Callback;

use crate::{Error, emit_callback_if_ok, MapErrAndLog};

pub async fn get_tables() -> Result<Vec<IdNamePair>, Error> {
    from_result(invoke_no_args("get_tables").await)
}

pub fn get_tables_with_callback(callback: impl Into<Callback<Vec<IdNamePair>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_tables(), callback.into()));
}

#[derive(Debug, Clone, Copy, Serialize)]
struct GetTableArgs {
    id: Uuid
}

pub async fn get_table(id: Uuid) -> Result<TableData, Error> {
    let args = serde_wasm_bindgen::to_value(&GetTableArgs { id }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    from_result(invoke("get_table", args).await)
}

pub fn get_table_with_callback(id: Uuid, callback: impl Into<Callback<TableData>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_table(id), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct NewTableArgs {
    #[serde(rename = "useCost")]
    use_cost: bool,
    #[serde(rename = "useWeight")]
    use_weight: bool,
    name: String,
    entries: Vec<TableEntry>
}

pub async fn new_table(use_cost: bool, use_weight: bool, name: impl Into<String>, entries: Vec<TableEntry>) -> Result<Uuid, Error> {
    let args = serde_wasm_bindgen::to_value(&NewTableArgs { use_cost, use_weight, name: name.into(), entries }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    let res = invoke("new_table", args).await;

    log::info!("Result: {:?}", &res);

    from_result(res)
}

pub fn new_table_with_callback(use_cost: bool, use_weight: bool, name: impl Into<String>, entries: Vec<TableEntry>, callback: impl Into<Callback<Uuid>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(new_table(use_cost, use_weight, name.into(), entries), callback.into()));
}

#[derive(Debug, Clone, Copy, Serialize)]
struct RemoveTableArgs {
    id: Uuid
}

pub async fn remove_table(id: Uuid) -> Result<TableData, Error> {
    let args = serde_wasm_bindgen::to_value(&RemoveTableArgs { id }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    from_result(invoke("remove_table", args).await)
}

pub fn remove_table_with_callback(id: Uuid, callback: impl Into<Callback<TableData>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(remove_table(id), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct ChangeTableNameArgs {
    id: Uuid,
    name: String
}

pub async fn change_table_name(id: Uuid, name: impl Into<String>) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&ChangeTableNameArgs { id, name: name.into() }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    unit_from_result(invoke("change_table_name", args).await)
}

pub fn change_table_name_with_callback(id: Uuid, name: impl Into<String>, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(change_table_name(id, name.into()), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct AddEntriesArgs {
    id: Uuid,
    entries: Vec<TableEntry>
}

pub async fn add_entries(id: Uuid, entries: Vec<TableEntry>) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&AddEntriesArgs { id, entries }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    unit_from_result(invoke("add_entries", args).await)
}

pub fn add_entries_with_callback(id: Uuid, entries: Vec<TableEntry>, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(add_entries(id, entries), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct RemoveEntryArgs {
    id: Uuid,
    index: usize
}

pub async fn remove_entry(id: Uuid, index: usize) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&RemoveEntryArgs { id, index }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    unit_from_result(invoke("remove_entry", args).await)
}

pub fn remove_entry_with_callback(id: Uuid, index: usize, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(remove_entry(id, index), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct GetRandomArgs {
    id: Uuid
}

pub async fn get_random(id: Uuid) -> Result<String, Error> {
    let args = serde_wasm_bindgen::to_value(&GetRandomArgs { id }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    from_result(invoke("get_random", args).await)
}

pub fn get_random_with_callback(id: Uuid, callback: impl Into<Callback<String>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_random(id), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct GetRandomSetArgs {
    id: Uuid,
    limit: RollLimit,
    #[serde(rename = "allowDuplicates")]
    allow_duplicates: bool
}

pub async fn get_random_set(id: Uuid, limit: RollLimit, allow_duplicates: bool) -> Result<Vec<RollResult>, Error> {
    let args = serde_wasm_bindgen::to_value(&GetRandomSetArgs { id, limit, allow_duplicates }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    from_result(invoke("get_random_set", args).await)
}

pub fn get_random_set_with_callback(id: Uuid, limit: RollLimit, allow_duplicates: bool, callback: impl Into<Callback<Vec<RollResult>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_random_set(id, limit, allow_duplicates), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct OpenTableArgs {
    path: PathBuf
}

pub async fn open_table(path: PathBuf) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&OpenTableArgs { path }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    unit_from_result(invoke("open_table", args).await)
}

pub fn open_table_with_callback(path: PathBuf, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(open_table(path), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct SaveTableArgs {
    id: Uuid,
    path: PathBuf
}

pub async fn save_table(id: Uuid, path: PathBuf) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&SaveTableArgs { id, path }).map_err_and_log(Error::SerdeWasmBindgenError)?;
    unit_from_result(invoke("save_table", args).await)
}

pub fn save_table_with_callback(id: Uuid, path: PathBuf, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(save_table(id, path), callback.into()));
}

#[derive(Debug, Clone, Serialize)]
struct DialogFilter {
    name: String,
    extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SaveDialogArgs {
    title: Option<String>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
    filters: Option<Vec<DialogFilter>>
}

#[derive(Debug, Clone, Serialize)]
struct OpenDialogArgs {
    title: Option<String>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
    filters: Option<Vec<DialogFilter>>,
    directory: bool,
    multiple: bool,
    recursive: bool,
}

pub async fn get_save_table_path() -> Result<Option<PathBuf>, Error> {
    let args = SaveDialogArgs {
        title: None,
        default_path: None,
        filters: Some(vec![
            DialogFilter {
                name: "Table".into(),
                extensions: vec!["table".into()]
            }
        ])
    };

    let args = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    serde_wasm_bindgen::from_value(save(args).await).map_err(Error::SerdeWasmBindgenError)
}

pub fn get_save_table_path_with_callback(callback: impl Into<Callback<Option<PathBuf>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_save_table_path(), callback.into()));
}

pub async fn get_open_table_path() -> Result<Option<PathBuf>, Error> {
    let args = OpenDialogArgs {
        title: None,
        default_path: None,
        directory: false,
        multiple: false,
        recursive: false,
        filters: Some(vec![
            DialogFilter {
                name: "Table".into(),
                extensions: vec!["table".into()]
            }
        ])
    };

    let args = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    serde_wasm_bindgen::from_value(open(args).await).map_err(Error::SerdeWasmBindgenError)
}

pub fn get_open_table_path_with_callback(callback: impl Into<Callback<Option<PathBuf>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_open_table_path(), callback.into()));
}

fn unit_from_result(result: Result<JsValue, JsValue>) -> Result<(), Error> {
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(from_error(err))
    }
}

fn from_result<T: for<'de> serde::Deserialize<'de>>(result: Result<JsValue, JsValue>) -> Result<T, Error> {
    match result {
        Ok(result) => from_value(result),
        Err(err) => Err(from_error(err))
    }
}

fn from_error(value: JsValue) -> Error {
    match serde_wasm_bindgen::from_value(value) {
        Ok(err) => Error::BackendError(err),
        Err(e) => {
            log::error!("Failed to convert error: {}", &e);
            Error::SerdeWasmBindgenError(e)
        }
    }
}

fn from_value<T: for<'de> serde::Deserialize<'de>>(value: JsValue) -> Result<T, Error> {
    serde_wasm_bindgen::from_value(value).map_err(|e| {
        log::error!("Failed to convert value: {}", &e);
        Error::SerdeWasmBindgenError(e)
    })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke", catch)]
    async fn invoke_no_args(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn save(args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(args: JsValue) -> JsValue;
}