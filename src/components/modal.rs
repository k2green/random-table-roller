use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub allow_scrolling: bool,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let ModalProps { children, allow_scrolling } = props.clone();
    
    let class = classes!("overlay", "overlay-colour", "stretch", "flex-column", "center-main-axis", "center-cross-axis");
    let class = if allow_scrolling {
        classes!(class, "scroll")
    } else {
        class
    };

    html! {
        <div class={class}>
            <div class="modal">
                {children}
            </div>
        </div>
    }
}