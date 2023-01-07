use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormTextBoxProps {
    pub id: AttrValue,
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or("text".into())]
    pub input_type: AttrValue,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
}

/// Text box that has `form-control` and `form-floating`.
/// Used in login and register pages.
#[function_component(FormTextBox)]
pub fn form_text_box(props: &FormTextBoxProps) -> Html {
    html! {
        <div class="form-floating mb-3">
            <input type={props.input_type.clone()}
            class="form-control" id={props.id.clone()} value={props.value.clone()}
            style="background-color: var(--bs-body-bg);"  // BUGFIX for Bootstrap 5.2.0 dark mode inputs
            oninput={props.oninput.clone()} placeholder={props.label.clone()} />
            <label for={props.id.clone()}>{props.label.clone()}</label>
        </div>
    }
}
