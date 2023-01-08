use yew::suspense::use_future;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::use_navigator;

use crate::api;

use crate::components::ModalLoadingSpinner;
use crate::Route;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let fallback = html! {
        <ModalLoadingSpinner text="Loading dashboard..." />
    };

    html! {
        <Suspense {fallback}>
            <DashboardInner />
        </Suspense>
    }
}

#[function_component(DashboardInner)]
fn dashboard_inner() -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let req_result = use_future(|| async {
        let res = api::auth_get_current_token().await;
        res
    });
    let req_result = req_result?;
    let result_html = match &*req_result {
        Ok(info) => {
            let username = &info.user.username;
            html! {
                <div>
                    <h1>{format!("Welcome, {}!", username)}</h1>
                </div>
            }
        }
        Err(_) => {
            spawn_local(async move {
                navigator.push(&Route::Login);
            });
            html! {
                <ModalLoadingSpinner text="Redirecting to login..." />
            }
        }
    };
    Ok(result_html)
}
