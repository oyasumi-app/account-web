use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;

use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::api::*;

use crate::components::CenteredBox;
use crate::components::FormSubmitBtn;
use crate::components::FormTextBox;
use crate::components::Size;
use crate::Route;
use api_types::v1::LoginResponse;
use web_sys::HtmlInputElement;

use crate::components::LoadingSpinner;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub login: String,
    pub password: String,
}

impl Into<api_types::v1::LoginRequest> for LoginInfo {
    fn into(self) -> api_types::v1::LoginRequest {
        if self.login.contains('@') {
            api_types::v1::LoginRequest::EmailPassword {
                email: self.login,
                password: self.password,
            }
        } else {
            api_types::v1::LoginRequest::UsernamePassword {
                username: self.login,
                password: self.password,
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[function_component(Login)]
pub fn login_page() -> Html {
    // Try logging in.
    let login_info = use_state(|| LoginInfo::default());
    let navigator = use_navigator().unwrap();
    let is_logging_in = use_state(|| false);

    let user_login = {
        let login_info = login_info.clone();
        let is_logging_in = is_logging_in.clone();
        use_async(async move {
            let request = (*login_info).clone().into();
            is_logging_in.set(true);
            let response = auth_login(request).await;
            match response {
                Ok(LoginResponse::Ok { token: _ }) => {
                    log::info!("Logged in!");

                    navigator.push(&Route::Dashboard);
                }
                _ => {
                    log::info!("Failed to log in!");
                    is_logging_in.set(false);
                    navigator.push(&Route::Login);
                }
            }
            Ok::<(), ()>(())
        })
    };

    let oninput_login = {
        let login_info = login_info.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.login = input.value();
            login_info.set(info);
        })
    };

    let oninput_pw = {
        let login_info = login_info.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    let onsubmit = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            user_login.run();
        })
    };

    html! {
        <CenteredBox>
            <h1>{ "Login" }</h1>
            <FormTextBox id="login" input_type="email" label="Username or Email" value={login_info.login.clone()} oninput={oninput_login} />
            <FormTextBox id="password" input_type="password" label="Password" value={login_info.password.clone()} oninput={oninput_pw} />
            <FormSubmitBtn onclick={onsubmit}>
                <LoadingSpinner show={*is_logging_in} size={Size::Small} />
                { "Login" }
            </FormSubmitBtn>

        </CenteredBox>
    }
}
