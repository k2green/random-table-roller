use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RemoveButtonProps {
    #[prop_or("17px".into())]
    pub width: AttrValue,
    #[prop_or("17px".into())]
    pub height: AttrValue,
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes
}

#[function_component(RemoveButton)]
pub fn remove_button(props: &RemoveButtonProps) -> Html {
    let RemoveButtonProps { on_click, class, width, height } = props.clone();
    let is_hovering = use_state_eq(|| false);

    let set_hovering = {
        let is_hovering = is_hovering.clone();
        Callback::from(move |_: MouseEvent| {
            is_hovering.set(true);
        })
    };

    let clear_hovering = {
        let is_hovering = is_hovering.clone();
        Callback::from(move |_: MouseEvent| {
            is_hovering.set(false);
        })
    };

    let icon_id = if *is_hovering {
        IconId::HeroiconsSolidMinusCircle
    } else {
        IconId::HeroiconsOutlineMinusCircle
    };

    html! {
        <button class={classes!("blank", class)} onclick={on_click} onmouseover={set_hovering} onmouseout={clear_hovering}>
            <Icon icon_id={icon_id} class="stroke-colour" width={width} height={height} />
        </button>
    }
}