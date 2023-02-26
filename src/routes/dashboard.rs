use std::rc::Rc;

use yew::suspense::use_future;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::use_navigator;

use crate::api;

use crate::components::{DashboardLayout, ModalLoadingSpinner};
use crate::context::UserContext;
use crate::Route;

mod home;
mod profile_page;
mod tabs;

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
        Ok(_) => {
            log::info!("Dashboard: Logged in with {:?}", context);

            html! {
                <ContextProvider<Rc<UserContext>> context={context}>
                <DashboardLayout>
                    <div class="row">
                        <div class="col-2">
                            <div class="nav flex-column nav-pills" id="v-pills-tab" role="tablist" aria-orientation="vertical">
                                <tabs::DashboardTabColumn />
                            </div>
                        </div>
                        <div class="col-10">
                            <div class="tab-content">
                                <tabs::DashboardTabContent />
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
