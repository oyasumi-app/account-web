use yew::prelude::*;

use super::Color;

#[derive(Properties, PartialEq)]
pub struct FormSubmitBtnProps {
    #[prop_or(Color::Primary)]
    pub color: Color,

    pub onclick: Callback<MouseEvent>,

    #[prop_or(true)]
    pub enabled: bool,
    pub children: Children,
}

/// Button `btn-lg` and `rounded-3`, used in login and register pages.
#[function_component(FormSubmitBtn)]
pub fn form_submit_button(props: &FormSubmitBtnProps) -> Html {
    if props.enabled {
        html! {
            <button type="submit" class={format!("w-100 mb-2 btn btn-lg btn-{} rounded-3", props.color)}
            onclick={props.onclick.clone()}>{for props.children.iter()}</button>
        }
    } else {
        html! {
            <button type="submit" disabled={true} class={format!("w-100 mb-2 btn btn-lg btn-{} rounded-3", props.color)}>
                {for props.children.iter()}
            </button>
        }
    }
}
