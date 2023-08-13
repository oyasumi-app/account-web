use std::future::Future;
use std::pin::Pin;
use yew::prelude::*;
use yew_hooks::use_async;

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

    #[prop_or(false)]
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
                    <LoadingSpinner show={false} inline={true} size={Size::Small} />
                    {&props.text}
                </button>
            }
        } else {
            html! {
                <button disabled={true} class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}
                >
                    <LoadingSpinner show={true} inline={true} size={Size::Small} />
                    {&props.text}
                </button>
            }
        }
    } else {
        html! {
            <button disabled={true} class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}>
                <LoadingSpinner show={false} inline={true} size={Size::Small} />
                {&props.text}
            </button>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AsyncButtonProps {
    #[prop_or(Color::Primary)]
    pub color: Color,

    #[prop_or(Size::Default)]
    pub size: Size,

    pub onclick_fn: Callback<(), Pin<Box<dyn Future<Output = ()>>>>,

    #[prop_or(true)]
    pub enabled: bool,

    pub text: AttrValue,
    pub class: Classes,
}

/// Button with customizable style and a spinner.
#[function_component(AsyncButton)]
pub fn async_button(props: &AsyncButtonProps) -> Html {
    let onclick_fn = props.onclick_fn.emit(());
    let my_fut = async move {
        onclick_fn.await;
        Ok::<(), ()>(())
    };
    let action = use_async(my_fut);
    let btn_cb = {
        let action = action.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            action.run();
        })
    };

    if props.enabled {
        if !action.loading {
            html! {
                <button class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}
                onclick={btn_cb}>
                    <LoadingSpinner show={false} inline={true} size={Size::Small} />
                    {&props.text}
                </button>
            }
        } else {
            html! {
                <button disabled={true} class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}
                >
                    <LoadingSpinner show={true} inline={true} size={Size::Small} />
                    {&props.text}
                </button>
            }
        }
    } else {
        html! {
            <button disabled={true} class={classes!(props.class.clone(), "btn", format!("btn-{}", props.color), props.size.class("btn"))}>
                <LoadingSpinner show={action.loading} inline={true} size={Size::Small}/>
                {&props.text}
            </button>
        }
    }
}
