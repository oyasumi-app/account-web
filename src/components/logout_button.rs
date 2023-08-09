use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::{
    api::auth_logout,
    components::{LoadingSpinner, Size},
    Route,
};

#[derive(Properties, PartialEq)]
pub struct LogoutButtonProps {
    pub class: Classes,
}

/// Link with spinner and text "Log out" that when clicked invalidates the token and redirects to the login page.
///
/// It's expected that this is placed in the [`DashboardNavbar`].
#[function_component(LogoutButton)]
pub fn logout_button(props: &LogoutButtonProps) -> Html {
    let is_logging_out = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let user_logout = {
        let is_logging_out = is_logging_out.clone();
        use_async(async move {
            is_logging_out.set(true);
            let response = auth_logout().await;
            match response {
                Ok(crate::api::ResponseType_auth_logout::Status204(_)) => {
                    log::info!("Logged out!");
                    navigator.push(&Route::Login);
                }
                _ => {
                    log::error!("Failed to log out!");
                    is_logging_out.set(false);
                }
            }
            Ok::<(), ()>(())
        })
    };

    let onclick = Callback::from(move |event: MouseEvent| {
        event.prevent_default();
        user_logout.run();
    });

    html! {
        <a class={props.class.clone()} onclick={onclick}>
            {"Log out"}
            <LoadingSpinner show={*is_logging_out} size={Size::Small} />
        </a>
    }
}
