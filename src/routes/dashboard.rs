use std::rc::Rc;

use yew::suspense::use_future;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::{use_navigator, use_route, Link};

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

    let cur_route: Route = use_route().expect("Route in DashboardInner expected to be one of the dashboard routes");

    let result_html = match &*req_result {
        Ok(info) => {
            let username = &info.user.username;
            log::info!("Dashboard: Logged in with {:?}", context);

            let routes = vec![
                (Route::Dashboard, "Home"),
                (Route::Dashboard1, "Tab 1"),
                (Route::Dashboard2, "Tab 2"),
                (Route::Dashboard3, "Tab 3"),
            ];

            let tabs = routes
                .iter()
                .map(|(route, name)| {
                    let active = cur_route == *route;
                    let maybe_active = if active { Some("active") } else { None };
                    html! {
                        <Link<Route> classes={classes!("nav-link", maybe_active)} to={route.clone()}> // role="tab" aria-selected="true"
                                {name}
                        </Link<Route>>
                    }
                })
                .collect::<Html>();

                let cur_tab_content = routes.iter().find(|(route, _)| cur_route == *route).map(|(_, name)| {
                    html! {
                        <div class="tab-pane fade show active" id="v-pills-home" role="tabpanel" aria-labelledby="v-pills-home-tab">
                            <h1>{name}</h1>
                            <p>{format!("Hello, {}!", username)}</p>
                        </div>
                    }
                }).unwrap();

            html! {
                <ContextProvider<Rc<UserContext>> context={context}>
                <DashboardLayout>
                    <div class="row">
                        <div class="col-2">
                            <div class="nav flex-column nav-pills" id="v-pills-tab" role="tablist" aria-orientation="vertical">
                                {tabs}
                            </div>
                        </div>
                        <div class="col-10">
                            <div class="tab-content">
                                {cur_tab_content}
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
