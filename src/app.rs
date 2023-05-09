use yew::prelude::*;

use crate::{components::{menu::Menu, table_tabs::TableTabs}, hooks::prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    let is_menu_open = use_state_eq(|| false);
    let tables = use_tables();

    html! {
        <div class="flex-row stretch no-scroll">
            <Menu is_open={is_menu_open}>
                <h2>{"Random table tool"}</h2>
            </Menu>
            <main class="flex-grow-1 stretch-height no-scroll">
                <TableTabs tables={tables.clone()} />
            </main>
        </div>
    }
}
