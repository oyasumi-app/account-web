use api_types::v1::{ConfirmRegistrationRequest, ConfirmRegistrationResponse};
use api_types::Snowflake;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use yew::suspense::use_future;
use yew_bootstrap::component::Alert;
use yew_bootstrap::util::Color;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::api::registration_confirm;
use crate::{api, Route};

use crate::components::style::Size;
use crate::components::CenteredBox;
use crate::components::FormSubmitBtn;
use crate::components::FormTextBox;
use crate::components::LoadingSpinner;

#[derive(Clone, PartialEq, Properties)]
pub struct ConfirmRegisterProps {
    pub registration_id: Snowflake,
}

#[function_component(ConfirmRegister)]
pub fn confirm_register(props: &ConfirmRegisterProps) -> Html {
    let fallback = html! {
        <Alert style={Color::Success}>{"Loading your registration..."}</Alert>
    };

    html! {
        <CenteredBox title="Confirm registration">
            <Suspense fallback={fallback}>
                <ConfirmInner registration_id={props.registration_id} />
            </Suspense>
        </CenteredBox>
    }
}

#[derive(Debug, Clone, Default)]
struct TokenInfo {
    pub token: String,
}

#[function_component(ConfirmInner)]
fn confirm_inner(props: &ConfirmRegisterProps) -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let registration_id = props.registration_id;
    let registration_result = use_future(|| async move {
        let reg = api::registration_get(registration_id).await;
        reg
    })?;

    let state_handle = registration_result.as_ref();
    let token_info = use_state(TokenInfo::default);
    let is_token_error = use_state(|| false);
    let is_button_enabled = use_state(|| true);
    let is_confirming = use_state(|| false);

    let confirm_token = {
        let token_info = token_info.clone();
        let is_confirming = is_confirming.clone();
        let is_token_error = is_token_error.clone();
        use_async(async move {
            is_confirming.set(true);
            is_token_error.set(false);

            let request = ConfirmRegistrationRequest {
                token: token_info.token.clone(),
            };
            let response = registration_confirm(registration_id, request).await;
            if let Ok(api::ResponseType_registration_confirm::Status200(response)) = response {
                match response {
                    ConfirmRegistrationResponse::Ok { token: _token } => {
                        navigator.push(&Route::DashboardHome);
                        return Ok(());
                    }
                    ConfirmRegistrationResponse::DatabaseError => (),
                    ConfirmRegistrationResponse::RegistrationConfirmError => (),
                    ConfirmRegistrationResponse::UserAlreadyExists => (),
                };
            };
            // If we fall through to here, then one of the errors occurred.
            is_token_error.set(true);
            is_confirming.set(false);
            Ok::<(), ()>(())
        })
    };

    let oninput = {
        let token_info = token_info.clone();
        let is_token_error = is_token_error.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*token_info).clone();
            info.token = input.value();
            token_info.set(info);
            is_token_error.set(false);
        })
    };

    let onsubmit = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            confirm_token.run();
        })
    };

    let maybe_error_alert = if *is_token_error {
        html! {
            <Alert style={Color::Danger}>{"Error confirming your token. Please check your email again."}</Alert>
        }
    } else {
        html! {}
    };

    let output = match state_handle {
        Err(_) => html! {
            <>
                <h1>{"Registration not found"}</h1>
                <p>{"We could not find a pending registration with this ID. It could have already expired."}</p>
                <p> {"You can "}
                <Link<Route> to={Route::Login}>{"login with an existing account"}</Link<Route>>{", or "}
                <Link<Route> to={Route::Register}>{"try registering again"}</Link<Route>>{"."}
                </p>
            </>
        },
        Ok(_registration) => html! {
            // TODO: this should be refactored into a separate component, but how to pass a PendingRegistration there?
            <form>
                {maybe_error_alert}
                <FormTextBox id="token" input_type="text" label="Confirmation" value={token_info.token.clone()} oninput={oninput} is_invalid={*is_token_error} />
                <FormSubmitBtn onclick={onsubmit} enabled={*is_button_enabled}>
                    <LoadingSpinner show={*is_confirming} size={Size::Small} />
                    { "Confirm registration" }
                </FormSubmitBtn>

            </form>
        },
    };

    Ok(output)
}
