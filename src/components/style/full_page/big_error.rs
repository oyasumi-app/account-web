use yew::prelude::*;

use crate::components::CenteredBox;

#[derive(Properties, PartialEq)]
pub struct BigErrorProps {
    #[prop_or("Unrecoverable Error".into())]
    pub short_name: AttrValue,

    #[prop_or_default]
    pub text: Option<AttrValue>,

    #[prop_or_default]
    pub what_to_do: Option<AttrValue>,

    #[prop_or_default]
    pub diagnostics: Option<AttrValue>,
}

/// A page layout for unrecoverable errors.
#[function_component(BigError)]
pub fn big_error(props: &BigErrorProps) -> Html {
    html! {
        <CenteredBox>
            <div class="d-flex flex-column justify-content-center align-content-center">

                // Error graphic
                <img class="mx-auto img-fluid" src="https://via.placeholder.com/400" alt="Error icon" />

                // Short name
                <h1 class="display-5">{props.short_name.clone()}</h1>

                // Text
                {if let Some(text) = &props.text {
                    html! {
                        <p class="lead">{text.clone()}</p>
                    }
                } else {
                    html! {}
                }}

                // What to do
                {if let Some(what_to_do) = &props.what_to_do {
                    html! {
                        <p>{what_to_do.clone()}</p>
                    }
                } else {
                    html! {}
                }}

                // Diagnostics
                {if let Some(diagnostics) = &props.diagnostics {
                    html! {<details>
                        <summary> {"Diagnostic information"} </summary>
                        <pre style="white-space: pre-wrap;"><code>{diagnostics.clone()}</code></pre>
                    </details>
                    }
                } else {
                    html! {}
                }}



            </div>
        </CenteredBox>

    }
}
