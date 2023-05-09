use std::sync::Arc;

use common_data::TableData;
use yew::prelude::*;

use crate::{hooks::prelude::UseTablesHandle, glue::remove_table_with_callback};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableTabsProps {
    pub tables: UseTablesHandle
}

#[function_component(TableTabs)]
pub fn table_tabs(props: &TableTabsProps) -> Html {
    let TableTabsProps { tables } = props.clone();

    let items = tables.tables()
        .iter()
        .enumerate()
        .map(|(idx, table)| {
            let set_index = {
                let tables = tables.clone();
                Callback::from(move |_: MouseEvent| {
                    tables.set_table_index(idx);
                })
            };

            let remove_table = {
                let tables = tables.clone();
                let id = tables.tables()[idx].id();

                Callback::from(move |_: MouseEvent| {
                    let tables = tables.clone();
                    remove_table_with_callback(id, move |_| {
                        tables.update()
                    });
                })
            };

            let class = match tables.get_selected_index() {
                Some(index) if index == idx => classes!("tab-button-disabled"),
                _ => classes!("tab-button")
            };

            html! {
                <div class={classes!(class, "flex-row")}>
                    <button onclick={set_index}>{table.name()}</button>
                    <button onclick={remove_table}>{"X"}</button>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="stretch flex-column no-scroll">
            <div class="flex-row scroll-x tab-bar">
                {items}
            </div>
            <div class="flex-grow-1 flex-column scroll tab-content">
                {get_table_content(tables.get_table_data())}
            </div>
        </div>
    }
}

fn get_table_content(table: Option<Arc<TableData>>) -> Html {
    match table {
        None => render_welcome_content(),
        Some(table) => html! {
            <TabContent table={table.clone()} />
        }
    }
}

fn render_welcome_content() -> Html {
    html! {
        <div class="flex-column flex-grow-1">
            <h1>{"Welcome!"}</h1>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct TabContentProps {
    table: Arc<TableData>,
}

#[function_component(TabContent)]
fn tab_content(props: &TabContentProps) -> Html {
    let TabContentProps { table } = props.clone();

    let entries = table.iter()
        .enumerate()
        .map(|(idx, entry)| {
            html! {
                <tr>
                    <td>{idx}</td>
                    <td>{entry}</td>
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <div class="flex-column flex-grow-1">
                <h1>{table.name()}</h1>
                <table>
                    <thead>
                        <tr>
                            <th>{"Roll"}</th>
                            <th>{"Entry"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {entries}
                    </tbody>
                </table>
            </div>
            <div class="flex-row button-row">
                <button class="flex-grow-1">{"Add entries"}</button>
                <button class="flex-grow-1">{"Roll"}</button>
            </div>
        </>
    }
}