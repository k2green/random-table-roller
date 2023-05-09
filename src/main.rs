mod app;
pub mod glue;
pub mod components;

use std::future::Future;

use app::App;
use yew::Callback;

pub(crate) async fn emit_callback_if_ok<T, E: std::fmt::Display, F: Future<Output = Result<T, E>>>(future: F, callback: Callback<T>) {
    match future.await {
        Ok(result) => callback.emit(result),
        Err(e) => log::error!("Failed to emit callback: {}", e)
    }
}

#[derive(Debug)]
pub enum Error {
    BackendError(common_data::BackendError),
    SerdeWasmBindgenError(serde_wasm_bindgen::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendError(e) => write!(f, "Backend error: {}", e),
            Self::SerdeWasmBindgenError(e) => write!(f, "Serde WASM bindgen error: {}", e),
        }
    }
}

pub trait MapErrAndLog<T, U> {
    fn map_err_and_log<V, F: Fn(U) -> V + 'static>(self, mapper: F) -> Result<T, V>;
}

impl<T, U: std::error::Error> MapErrAndLog<T, U> for Result<T, U> {
    fn map_err_and_log<V, F: Fn(U) -> V + 'static>(self, mapper: F) -> Result<T, V> {
        self.map_err(move |e| {
            log::error!("{}", e);
            mapper(e)
        })
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
