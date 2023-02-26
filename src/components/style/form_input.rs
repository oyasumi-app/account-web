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

    #[prop_or_default]
    pub is_invalid: bool,
}

/// Text box that has `form-control` and `form-floating`.
/// Used in login and register pages.
#[function_component(FormTextBox)]
pub fn form_text_box(props: &FormTextBoxProps) -> Html {
    let item_classes = classes!(
        "form-control",
        if props.is_invalid {
            Some("is-invalid")
        } else {
            None
        },
    );
    html! {
        <div class="form-floating mb-3">
            <input type={props.input_type.clone()}
            class={item_classes} id={props.id.clone()} value={props.value.clone()}
            style="background-color: var(--bs-body-bg);"  // BUGFIX for Bootstrap 5.2.0 dark mode inputs
            oninput={props.oninput.clone()} placeholder={props.label.clone()} />
            <label for={props.id.clone()}>{props.label.clone()}</label>
        </div>
    }
}
