use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FullPageModalProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(true)]
    pub allow_scrolling: bool
}

#[function_component(FullPageModal)]
pub fn full_page_modal(props: &FullPageModalProps) -> Html {
    let FullPageModalProps { children, allow_scrolling } = props.clone();

    let class = classes!("overlay", "modal-full", "flex-column");
    let class = if allow_scrolling {
        classes!(class, "scroll")
    } else {
        classes!(class, "no-scroll")
    };

    html! {
        <div class={class}>
            <div class="content flex-column">
                {children}
            </div>
        </div>
    }
}