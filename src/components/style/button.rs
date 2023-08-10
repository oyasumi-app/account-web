use yew::prelude::*;

use super::super::LoadingSpinner;
use super::{Color, Size};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(Color::Primary)]
    pub color: Color,

    #[prop_or(Size::Default)]
    pub size: Size,

    pub onclick: Callback<()>,

    #[prop_or(true)]
    pub enabled: bool,

    #[prop_or(true)]
    pub spinning: bool,

    pub text: AttrValue,
    pub class: Classes,
}

/// Button with customizable style and a spinner.
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let user_cb = props.onclick.clone();
    let btn_cb = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            user_cb.emit(());
        })
    };

    if props.enabled {
        if !props.spinning {
            html! {
                <button class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}
                onclick={btn_cb}>
                    <LoadingSpinner show={false} />
                    {&props.text}
                </button>
            }
        } else {
            html! {
                <button disabled={true} class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}
                >
                    <LoadingSpinner show={true} />
                    {&props.text}
                </button>
            }
        }
    } else {
        html! {
            <button disabled={true} class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}>
                <LoadingSpinner show={false} />
                {&props.text}
            </button>
        }
    }
}
