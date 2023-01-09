use serde::Deserialize;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::api;

use crate::Route;

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
}

impl From<RegisterInfo> for api_types::v1::RegistrationRequest {
    fn from(val: RegisterInfo) -> Self {
        api_types::v1::RegistrationRequest {
            username: val.username,
            email: val.email,
            password: val.password,
        }
    }
}


#[function_component(Register)]
pub fn register() -> Html {
    let register_info = use_state(RegisterInfo::default);
    let is_registering = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let perform_registration = {
        let register_info = register_info.clone();
        let is_registering = is_registering.clone();
        use_async(async move {
            let request = (*register_info).clone();
            is_registering.set(true);
            let response = api::auth_register(request.into()).await;
            is_registering.set(false);
            match response {
                Ok(api_types::v1::RegistrationResponse::Ok { id }) => {
                    navigator.push(&Route::ConfirmRegister { id });
                }
                Ok(api_types::v1::RegistrationResponse::PendingRegistrationExists { id }) => {
                    navigator.push(&Route::ConfirmRegister { id });
                }
                Ok(api_types::v1::RegistrationResponse::DatabaseError) => {
                    // TODO: retry?
                },
                Err(_) => {
                    // TODO: retry?
                }
            }

            Ok::<(), ()>(())
        })
    };

    let oninput_username = {
        let register_info = register_info.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info);
        })
    };

    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };

    let onsubmit = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            perform_registration.run();
        })
    };


    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info);
        })
    };


    html! {
        <CenteredBox title={"Registration"} >
            <form>
                <FormTextBox id="username" input_type="text" label="Username" value={register_info.username.clone()} oninput={oninput_username} />
                <FormTextBox id="email" input_type="email" label="Email" value={register_info.email.clone()} oninput={oninput_email} />
                <FormTextBox id="password" input_type="password" label="Password" value={register_info.password.clone()} oninput={oninput_password} />
                <FormSubmitBtn onclick={onsubmit}>
                    <LoadingSpinner show={*is_registering} size={Size::Small} />
                    { "Register" }
                </FormSubmitBtn>
            </form>

        </CenteredBox>
    }
}
