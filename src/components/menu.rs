use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuProps {
    pub is_open: UseStateHandle<bool>,
    #[prop_or_default]
    pub children: Children
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let MenuProps { is_open, children } = props.clone();
    let open_menu = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(true);
        })
    };

    let close_menu = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
        })
    };

    html! {
        <>
            <div class="stretch-height menu-bar">
                <button class="blank" onclick={open_menu}>
                    <Icon icon_id={IconId::LucideMenu} class="fill-colour" width="20px" height="20px" />
                </button>
            </div>
            if *is_open {
                {get_overlay_html(close_menu, children)}
            }
        </>
    }
}

fn get_overlay_html(close_menu: Callback<MouseEvent>, children: Children) -> Html {
    html! {
        <div class="overlay overlay-colour stretch flex-row">
            <div class="stretch-height flex-row">
                <div class="menu-content flex-column stretch-height">
                    {children}
                </div>
                <div class="stretch-height menu-bar">
                    <button class="blank" onclick={close_menu.clone()}>
                        <Icon icon_id={IconId::LucideMenu} class="fill-colour" width="20px" height="20px" />
                    </button>
                </div>
            </div>
            <div class="stretch flex-grow-1" onclick={close_menu} />
        </div>
    }
}