use serde::Deserialize;
use serde::Serialize;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::Node;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_bootstrap::component::Alert;
use yew_bootstrap::util::Color;
use yew_hooks::use_async;
use yew_hooks::use_is_first_mount;
use yew_router::prelude::*;

use crate::api;

use crate::Route;

use crate::api::registration_get_prerequisites;
use crate::components::CenteredBox;
use crate::components::FormSubmitBtn;
use crate::components::FormTextBox;
use crate::components::LoadingSpinner;
use crate::components::Size;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub hcaptcha_response: String,
}

pub enum RegisterError {
    EmailNotValid,
}

impl TryFrom<RegisterInfo> for api_types::v1::RegistrationRequest {
    type Error = RegisterError;

    fn try_from(val: RegisterInfo) -> Result<Self, Self::Error> {
        Ok(api_types::v1::RegistrationRequest {
            username: val.username,
            email: val
                .email
                .parse()
                .map_err(|_| RegisterError::EmailNotValid)?,
            password: val.password,
            hcaptcha_response: val.hcaptcha_response,
        })
    }
}

fn validate_user(info: &RegisterInfo) -> bool {
    !info.username.is_empty()
}

fn validate_password(info: &RegisterInfo) -> bool {
    !info.password.is_empty()
}

fn validate_email(info: &RegisterInfo) -> bool {
    let maybe_reg_req: Result<api_types::v1::RegistrationRequest, _> = info.clone().try_into();
    maybe_reg_req.is_ok()
}
#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <Suspense fallback={html!(
            <CenteredBox title={"Registration"} >
                <Alert style={Color::Success}>
                    <LoadingSpinner size={Size::Small} inline={true} />
                    {"Loading info for registration..."}
                </Alert>
            </CenteredBox>
        )}>
            <RegisterInner />
        </Suspense>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=hcaptcha, js_name=getResponse)]
    fn get_hcaptcha_response(widget_id: String) -> String;

    #[wasm_bindgen(js_namespace=hcaptcha, js_name=render)]
    fn render_hcaptcha(container: HtmlElement) -> String;

}

#[function_component(RegisterInner)]
pub fn register_inner() -> HtmlResult {
    let registration_prereqs = use_future(|| async { registration_get_prerequisites().await })?;

    let prereqs = match &*registration_prereqs {
        Ok(resp) => match resp {
            api::ResponseType_registration_get_prerequisites::Status200(resp) => resp.clone(),
        },
        Err(e) => {
            return Ok(html!(
                <CenteredBox title={"Registration"} >
                    <Alert style={Color::Danger}>
                        {"Failed loading info for registration. "}{e}
                    </Alert>
                </CenteredBox>
            ))
        }
    };

    let register_info = use_state(RegisterInfo::default);
    let is_registering = use_state(|| false);
    let is_username_error = use_state(|| true);
    let is_email_error = use_state(|| true);
    let is_password_error = use_state(|| true);
    let is_button_enabled = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let validate_info = |info: &RegisterInfo| {
        validate_email(info) && validate_password(info) && validate_user(info)
    };

    let perform_registration = {
        let register_info = register_info.clone();
        let is_registering = is_registering.clone();
        let is_button_enabled = is_button_enabled.clone();
        let is_email_error = is_email_error.clone();
        use_async(async move {
            let request = (*register_info).clone();
            is_registering.set(true);
            is_button_enabled.set(true);
            let maybe_request = request.try_into();
            match maybe_request {
                Ok(request) => {
                    let response = api::auth_register(request).await;
                    is_registering.set(false);
                    match response {
                        Ok(api::ResponseType_auth_register::Status200(resp)) => match resp {
                            api_types::v1::RegistrationResponse::Ok { id } => {
                                navigator.push(&Route::ConfirmRegister { id });
                            }
                            api_types::v1::RegistrationResponse::PendingRegistrationExists {
                                id,
                            } => {
                                navigator.push(&Route::ConfirmRegister { id });
                            }
                            api_types::v1::RegistrationResponse::DatabaseError => {
                                // TODO: retry?
                            }
                            api_types::v1::RegistrationResponse::HcaptchaFailure { error } => {
                                log::error!("Hcaptcha Error received: {error}");
                            }
                        },
                        Err(_) => {
                            // TODO: retry?
                        }
                    }
                }
                Err(err) => match err {
                    RegisterError::EmailNotValid => {
                        // Highlight the email input
                        is_email_error.set(true);
                    }
                },
            };

            Ok::<(), ()>(())
        })
    };

    let oninput_username = {
        let register_info = register_info.clone();
        let is_button_enabled = is_button_enabled.clone();
        let is_username_error = is_username_error.clone();
        let is_password_error = is_password_error.clone();
        let is_email_error = is_email_error.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            is_username_error.set(!validate_user(&info));
            is_password_error.set(!validate_password(&info));
            is_email_error.set(!validate_email(&info));
            is_button_enabled.set(validate_info(&info));
            register_info.set(info);
        })
    };

    let oninput_email = {
        let register_info = register_info.clone();
        let is_button_enabled = is_button_enabled.clone();
        let is_username_error = is_username_error.clone();
        let is_password_error = is_password_error.clone();
        let is_email_error = is_email_error.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            is_username_error.set(!validate_user(&info));
            is_password_error.set(!validate_password(&info));
            is_email_error.set(!validate_email(&info));
            is_button_enabled.set(validate_info(&info));
            register_info.set(info);
        })
    };

    let widget_id = use_state(|| String::new());

    let onsubmit = {
        let register_info = register_info.clone();
        let widget_id = widget_id.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let widget_id = (*widget_id).clone();
            #[allow(unused_unsafe)]
            let hcaptcha_response = unsafe { get_hcaptcha_response(widget_id) };
            let mut info = (*register_info).clone();
            info.hcaptcha_response = hcaptcha_response;
            register_info.set(info);
            perform_registration.run();
        })
    };

    let oninput_password = {
        let register_info = register_info.clone();
        let is_button_enabled = is_button_enabled.clone();
        let is_username_error = is_username_error.clone();
        let is_password_error = is_password_error.clone();
        let is_email_error = is_email_error.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            is_username_error.set(!validate_user(&info));
            is_password_error.set(!validate_password(&info));
            is_email_error.set(!validate_email(&info));
            is_button_enabled.set(validate_info(&info));
            register_info.set(info);
        })
    };

    let get_widget_id_cb = {
        let widget_id = widget_id.clone();
        Callback::from(move |new_id: String| {
            widget_id.set(new_id);
        })
    };

    Ok(html! {
        <CenteredBox title={"Registration"} >
            <form>
                <FormTextBox id="username" input_type="text" label="Username" value={register_info.username.clone()} oninput={oninput_username} is_invalid={*is_username_error} />
                <FormTextBox id="email" input_type="email" label="Email" value={register_info.email.clone()} oninput={oninput_email} is_invalid={*is_email_error} />
                <FormTextBox id="password" input_type="password" label="Password" value={register_info.password.clone()} oninput={oninput_password} is_invalid={*is_password_error} />
                <HcaptchaContainer sitekey={prereqs.hcaptcha_sitekey} get_widget_id={get_widget_id_cb} />
                <FormSubmitBtn onclick={onsubmit} enabled={*is_button_enabled}>
                    <LoadingSpinner show={*is_registering} size={Size::Small} />
                    { "Register" }
                </FormSubmitBtn>
            </form>
        <p> {"Or "} <Link<Route> to={Route::Login}>{"login using an existing account"}</Link<Route>>{" instead?"}</p>

        </CenteredBox>
    })
}

#[derive(PartialEq, Clone, Properties)]
struct HcaptchaContainerProps {
    sitekey: AttrValue,
    get_widget_id: Callback<String>,
}

#[function_component(HcaptchaContainer)]
fn hcaptcha_container(props: &HcaptchaContainerProps) -> Html {
    let is_first = use_is_first_mount();
    let hcaptcha_box_ref = use_node_ref();
    use_effect_with_deps(
        |(box_ref, is_first, get_id)| {
            if *is_first {
                let div = box_ref
                    .cast::<HtmlElement>()
                    .expect("hcaptcha_box_ref not attached to div element");
                #[allow(unused_unsafe)]
                let widget_id = unsafe { render_hcaptcha(div) };
                get_id.emit(widget_id);
            }
        },
        (
            hcaptcha_box_ref.clone(),
            is_first,
            props.get_widget_id.clone(),
        ),
    );

    html!(
        <div class="h-captcha" data-sitekey={&props.sitekey} data-theme="dark" ref={hcaptcha_box_ref} />
    )
}
