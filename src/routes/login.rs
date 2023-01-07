use serde::Deserialize;
use serde::Serialize;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::api::*;

use crate::Route;
use api_types::v1::LoginResponse;
use web_sys::HtmlInputElement;

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

    let user_login = {
        let login_info = login_info.clone();
        use_async(async move {
            let request = (*login_info).clone().into();
            let response = auth_login(request).await;
            match response {
                Ok(LoginResponse::Ok { token }) => {
                    log::info!("Logged in!");

                    navigator.push(&Route::Dashboard);
                }
                _ => {
                    log::info!("Failed to log in!");
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
        <div>
            <h1>{ "Login" }</h1>
            <input type="text" placeholder="Username" value={login_info.login.clone()} oninput={oninput_login} />
            <input type="password" placeholder="Password" value={login_info.password.clone()} oninput={oninput_pw} />
            <button onclick={onsubmit}>{ "Login" }</button>
        </div>
    }
}
