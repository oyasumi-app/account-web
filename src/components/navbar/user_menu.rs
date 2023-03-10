use std::rc::Rc;

use api_types::v1::TokenData;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{components::LogoutButton, context::UserContext, Route};

/// A component that displays a user menu as a dropdown.
/// The menu contains a link to the user's profile, and a logout button.
#[function_component(UserMenu)]
pub fn user_menu() -> Html {
    let user_ctx = use_context::<Rc<UserContext>>();
    if user_ctx.is_none() {
        log::error!("UserMenu: UserContext is not set!");
        return html! {};
    }
    let user_ctx = user_ctx.unwrap();

    match user_ctx.as_ref() {
        UserContext::LoggedOut => html! {},
        UserContext::LoggedIn(TokenData { user, token: _ }) => html! {
            <div class="dropdown">
                <button class="btn btn-dark dropdown-toggle" data-bs-display="static" data-bs-toggle="dropdown" aria-expanded="false">
                    {&user.username}
                </button>
                <ul class="dropdown-menu dropdown-menu-end">
                    <li><Link<Route> classes="dropdown-item" to={Route::DashboardProfile}>{"Profile"}</Link<Route>></li>
                    <li><LogoutButton class="dropdown-item" /></li>
                </ul>
            </div>
        },
    }
}
