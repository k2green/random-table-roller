use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let ModalProps { children } = props.clone();
    
    html! {
        <div class="overlay overlay-colour stretch flex-column center-main-axis center-cross-axis">
            <div class="modal">
                {children}
            </div>
        </div>
    }
}