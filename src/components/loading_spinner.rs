use yew::prelude::*;

use super::Size;

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or(true)]
    pub show: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or(true)]
    pub inline: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

/// Creates a loading spinner.
/// Takes a boolean to determine whether to show the spinner or not.
#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingProps) -> Html {
    if props.show {
        if props.inline {
            html! {
                <span style={props.style.clone()} class={classes!("spinner-border", props.size.class("spinner-border"))} role="status">
                    <span class="visually-hidden">{"Loading..."}</span>
                </span>
            }
        } else {
            html! {
                <div class="d-flex justify-content-center">
                    <span style={props.style.clone()} class={classes!("spinner-border", props.size.class("spinner-border"))} role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </span>
                </div>
            }
        }
    } else if props.inline {
        html! {
            <span class={classes!("spinner-border", props.size.class("spinner-border"))} role="status" style={format!("visibility: hidden; {}", props.style)}>
            </span>
        }
    } else {
        html! {
            <div class="d-flex justify-content-center" style="visibility: hidden">
                <span class={classes!("spinner-border", props.size.class("spinner-border"))} role="status" style={props.style.clone()}>
                </span>
            </div>
        }
    }
}
