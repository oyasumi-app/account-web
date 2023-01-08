use yew::prelude::*;

use crate::components::{CenteredBox, LoadingSpinner, Size};

#[derive(Properties, PartialEq)]
pub struct ModalLoadingSpinnerProps {
    #[prop_or("Loading...".into())]
    pub text: AttrValue,
}

/// A page layout consisting only of a large spinner and a text label inside a `CenteredBox`.
#[function_component(ModalLoadingSpinner)]
pub fn centered_box(props: &ModalLoadingSpinnerProps) -> Html {
    html! {
        <CenteredBox>
            <div class="d-flex flex-column justify-content-center align-content-center">
                <div class="mb-3"><LoadingSpinner style="width: 3rem; height: 3rem;" size={Size::Large} inline={false} /></div>
                <h4>{props.text.clone()}</h4>
            </div>
        </CenteredBox>

    }
}
