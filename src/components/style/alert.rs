use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct AlertProps {
    pub message: AttrValue,
}

#[function_component(DangerAlert)]
pub fn danger_alert(props: &AlertProps) -> Html {
    html! {
        <div class="alert alert-danger" role="alert">
            {props.message.clone()}
        </div>
    }
}
