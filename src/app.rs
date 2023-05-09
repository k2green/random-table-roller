use yew::prelude::*;

use crate::components::menu::Menu;

#[function_component(App)]
pub fn app() -> Html {
    let is_menu_open = use_state_eq(|| false);

    html! {
        <div class="flex-row stretch no-scroll">
            <Menu is_open={is_menu_open}>
                <h2>{"Random table tool"}</h2>
            </Menu>
            <main class="flex-grow-1 stretch-height">
                
            </main>
        </div>
    }
}
