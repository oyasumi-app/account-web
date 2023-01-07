use yew::prelude::*;

use super::Size;

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or(true)]
    pub show: bool,

    #[prop_or_default]
    pub size: Size,
}

/// Creates a loading spinner.
/// Takes a boolean to determine whether to show the spinner or not.
#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingProps) -> Html {
    if props.show {
        html! {
            <span class={classes!("spinner-border", props.size.class("spinner-border"))} role="status">
                <span class="visually-hidden">{"Loading..."}</span>
            </span>
        }
    } else {
        html! {
            <span class={classes!("spinner-border", props.size.class("spinner-border"))} role="status" style="visibility: hidden;">
            </span>
        }
    }
}
