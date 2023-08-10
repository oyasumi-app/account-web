use std::{borrow::Borrow, rc::Rc};

use api_types::v1::TokenData;
use yew::prelude::*;

use crate::components::current_sleep_status_component::CurrentSleepStatusComponent;
use crate::context::UserContext;

#[function_component(DashboardHome)]
pub fn dashboard_home() -> Html {
    let user_ctx_rc =
        use_context::<Rc<UserContext>>().expect("called dashboard_home without a UserContext");
    let user_context = user_ctx_rc.borrow();
    let (user, _token) = if let UserContext::LoggedIn(TokenData { user, token }) = user_context {
        (user, token)
    } else {
        panic!("called dashboard_home with a UserContext that is not logged in");
    };
    html! {
        <>
            <h1>{format!("Welcome, {}!", user.username)}</h1>
            <CurrentSleepStatusComponent />
        </>
    }
}
