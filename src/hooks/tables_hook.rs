use std::sync::Arc;

use common_data::{IdNamePair, TableData};
use yew::prelude::*;

use crate::glue::{get_tables_with_callback, get_table_with_callback};

#[derive(Debug, Clone, PartialEq)]
pub struct UseTablesHandle {
    update_state: UseStateHandle<bool>,
    tables: UseStateHandle<Vec<IdNamePair>>,
    table_index: UseStateHandle<Option<usize>>,
    table_data: UseStateHandle<Option<Arc<TableData>>>
}

impl UseTablesHandle {
    pub fn set_table_index(&self, index: usize) {
        if index <= self.tables.len() {
            self.table_index.set(Some(index))
        }
    }

    pub fn update(&self) {
        self.update_state.set(!*self.update_state);
    }

    pub fn tables(&self) -> &[IdNamePair] {
        &*self.tables
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        *self.table_index
    }

    pub fn get_table_data(&self) -> Option<Arc<TableData>> {
        (*self.table_data).clone()
    }
}

#[hook]
pub fn use_tables() -> UseTablesHandle {
    let update_state = use_state_eq(|| false);
    let tables = use_state_eq(|| Vec::new());
    let table_index = use_state_eq(|| None);
    let table_data = use_state_eq(|| None);

    use_effect_with_deps({
        let tables = tables.clone();
        let table_index = table_index.clone();

        move |_| {
            let tables = tables.clone();
            let table_index = table_index.clone();
            get_tables_with_callback(move |updated: Vec<IdNamePair>| {
                log::info!("Retrieved tables:\n{:#?}", &updated);

                let new_index = if updated.len() == 0 {
                    None
                } else {
                    match *table_index {
                        Some(index) if index >= updated.len() => Some(updated.len() - 1),
                        _ => *table_index
                    }
                };

                tables.set(updated);
                table_index.set(new_index);
            })
        }
    }, update_state.clone());

    use_effect_with_deps({
        let tables = tables.clone();
        let table_index = table_index.clone();
        let table_data = table_data.clone();

        move |_| {
            let tables = tables.clone();
            let table_index = table_index.clone();
            let table_data = table_data.clone();

            match *table_index {
                None => { table_data.set(None); },
                Some(index) => {
                    let pair: &IdNamePair = &tables[index];
                    let id = pair.id();

                    get_table_with_callback(id, move |table| {
                        log::info!("Retrieved table:\n{:#?}", &table);
                        table_data.set(Some(Arc::new(table)));
                    });
                }
            };
        }
    }, table_index.clone());

    UseTablesHandle { update_state, tables, table_index, table_data }
}