use std::rc::Rc;

use yew::suspense::use_future;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::use_navigator;

use crate::api;

use crate::components::{DashboardLayout, ModalLoadingSpinner};
use crate::context::UserContext;
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

    let context = use_memo(
        |_| match &*req_result {
            Ok(info) => UserContext::LoggedIn(info.clone()),
            Err(_) => UserContext::LoggedOut,
        },
        (),
    );

    let result_html = match &*req_result {
        Ok(info) => {
            let username = &info.user.username;
            log::info!("Dashboard: Logged in with {:?}", context);
            html! {
                <ContextProvider<Rc<UserContext>> context={context}>
                <DashboardLayout>
                    <div class="row">
                        <div class="col-2">
                            <div class="nav flex-column nav-pills" id="v-pills-tab" role="tablist" aria-orientation="vertical">
                                <a class="nav-link active" id="v-pills-home-tab" data-bs-toggle="pill" href="#v-pills-home" role="tab" aria-controls="v-pills-home" aria-selected="true">{"Home"}</a>
                                <a class="nav-link" id="v-pills-profile-tab" data-bs-toggle="pill" href="#v-pills-profile" role="tab" aria-controls="v-pills-profile" aria-selected="false">{"Profile"}</a>
                                <a class="nav-link" id="v-pills-messages-tab" data-bs-toggle="pill" href="#v-pills-messages" role="tab" aria-controls="v-pills-messages" aria-selected="false">{"Messages"}</a>
                                <a class="nav-link" id="v-pills-settings-tab" data-bs-toggle="pill" href="#v-pills-settings" role="tab" aria-controls="v-pills-settings" aria-selected="false">{"Settings"}</a>
                            </div>
                        </div>
                        <div class="col-10">
                            <div class="tab-content" id="v-pills-tabContent">
                                <div class="tab-pane fade show active" id="v-pills-home" role="tabpanel" aria-labelledby="v-pills-home-tab">
                                    <h1>{format!("Welcome, {username}!")}</h1>
                                </div>
                                <div class="tab-pane fade" id="v-pills-profile" role="tabpanel" aria-labelledby="v-pills-profile-tab">{"..."}</div>
                                <div class="tab-pane fade" id="v-pills-messages" role="tabpanel" aria-labelledby="v-pills-messages-tab">{"..."}</div>
                                <div class="tab-pane fade" id="v-pills-settings" role="tabpanel" aria-labelledby="v-pills-settings-tab">{"..."}</div>
                            </div>
                        </div>
                    </div>

                </DashboardLayout>
                </ContextProvider<Rc<UserContext>>>
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
