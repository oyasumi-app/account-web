use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::use_navigator;

use crate::api::*;

use crate::Route;

use crate::components::ModalLoadingSpinner;

#[function_component(CheckLoginRequired)]
pub fn check_login_required() -> Html {
    // Perform an API call to check if the user is logged in.
    // If they are, redirect to the dashboard.
    // If they are not, redirect to the login page.

    let navigator = use_navigator().unwrap();

    spawn_local(async move {
        let response = auth_check().await;

        match response {
            Ok(ResponseType_auth_check::Status200(api_types::v1::CheckResponse::ValidToken(_))) => {
                navigator.push(&Route::DashboardHome);
            }
            _ => {
                navigator.push(&Route::Login);
            }
        }
    });

    html! {
        <ModalLoadingSpinner text="Checking for existing login..." />
    }
}
