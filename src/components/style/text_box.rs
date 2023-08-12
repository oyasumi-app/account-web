use std::{future::Future, pin::Pin};

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

#[derive(Properties, PartialEq)]
pub struct AsyncTextBoxProps {
    pub id: AttrValue,

    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or("text".into())]
    pub input_type: AttrValue,

    #[prop_or("".into())]
    pub prefill: AttrValue,

    pub onchange_fn: Callback<(), Pin<Box<dyn Future<Output = AsyncTextBoxValidationResult>>>>,

    #[prop_or(false)]
    pub disabled: bool,
}

#[derive(Clone, Debug)]
pub enum AsyncTextBoxValidationResult {
    /// The value the user entered is okay, and has been saved.
    Ok,

    /// The value the user entered is invalid. The text explains why.
    Invalid(String),
}

/// Text box that accepts an async function to run on changes.
///
#[function_component(AsyncTextBox)]
pub fn form_text_box(props: &AsyncTextBoxProps) -> Html {
    let content = use_state(|| String::new());
    {
        let content = content.clone();
        use_effect_with_deps(
            move |prefill| content.set(prefill.to_string()),
            props.prefill.clone(),
        )
    }

    let onchange = props.onchange_fn.emit(());
    let state = use_state(|| Some(AsyncTextBoxValidationResult::Ok));

    let my_fut = {
        let state = state.clone();
        async move {
            let res = onchange.await;
            state.set(Some(res.clone()));
            Ok::<AsyncTextBoxValidationResult, ()>(res)
        }
    };

    let action = use_async(my_fut);

    let oninput_cb = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            state.set(None);
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value();
        })
    };

    let onchange_cb = {
        let action = action.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            action.run();
        })
    };

    let item_classes = classes!(
        "form-control",
        match &*state {
            Some(state) => match state {
                AsyncTextBoxValidationResult::Ok => None,
                AsyncTextBoxValidationResult::Invalid(_) => Some("is-invalid"),
            },
            None => Some("border-warning"),
        }
    );
    let validation_label = if let Some(AsyncTextBoxValidationResult::Invalid(feedback)) = &*state {
        html!(<div class="invalid-feedback">{feedback}</div>)
    } else {
        html!()
    };

    html! {
        <div class="form-floating mb-3">
            <input type={props.input_type.clone()}
            class={item_classes} id={props.id.clone()} value={(*content).clone()}
            style="background-color: var(--bs-body-bg);"  // BUGFIX for Bootstrap 5.2.0 dark mode inputs
            oninput={oninput_cb} onchange={onchange_cb}
            placeholder={props.label.clone()} />
            <label for={props.id.clone()}>{props.label.clone()}</label>
            {validation_label}
        </div>
    }
}
