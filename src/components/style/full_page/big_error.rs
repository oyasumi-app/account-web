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

/// This function returns a String containing HTML
/// that would be equivalent to the [`BigError`] component.
/// This is used when the panic hook is triggered:
/// it should use the same HTML as the `BigError` component,
/// but it can't use the component because it's in the panic hook.
///
/// TODO: At present, this duplicates code from the [`BigError`] component,
/// as well as the [`CenteredBox`] component.
/// Can we refactor this so that the component's implementation is used?
pub fn big_error_when_panic(title: &str, props: &BigErrorProps) -> String {
    let mut html = String::new();

    // Centered box begin
    html.push_str(&format!("<div class=\"container container-centered full-height\">
    <div class=\"modal modal-sheet d-block\">
        <div class=\"modal-dialog modal-dialog-centered\" role=\"document\" style=\"max-width: 80%\">
            <div class=\"modal-content rounded-4 shadow\">
                <div class=\"modal-header p-5 pb-4 border-bottom-0\">
                    <h1 class=\"fw-bold mb-0 fs-2\">
                        {title}
                    </h1>
                </div>
                <div class=\"modal-body p-5 pt-0\"
                style=\"padding: 1rem;\">"
));

    html.push_str("<div class=\"d-flex flex-column justify-content-center align-content-center\">");

    // Error graphic
    html.push_str("<img class=\"mx-auto img-fluid\" src=\"https://via.placeholder.com/400\" alt=\"Error icon\" />");

    // Short name
    html.push_str("<h1 class=\"display-5\">");
    html.push_str(&props.short_name);
    html.push_str("</h1>");

    // Text
    if let Some(text) = &props.text {
        html.push_str("<p class=\"lead\">");
        html.push_str(text);
        html.push_str("</p>");
    }

    // What to do
    if let Some(what_to_do) = &props.what_to_do {
        html.push_str("<p>");
        html.push_str(what_to_do);
        html.push_str("</p>");
    }

    // Diagnostics
    if let Some(diagnostics) = &props.diagnostics {
        html.push_str("<details>");
        html.push_str("<summary> Diagnostic information </summary>");
        html.push_str("<pre style=\"white-space: pre;\"><code>");
        html.push_str(diagnostics);
        html.push_str("</code></pre>");
        html.push_str("</details>");
    }

    html.push_str("</div>");

    // Centered box end
    html.push_str("</div></div></div></div></div>");

    html
}
