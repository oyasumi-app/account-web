use yew::prelude::*;
use yew_router::prelude::{use_route, Link};

use crate::Route;

#[function_component(DashboardTabContent)]
pub fn get_dashboard_tab() -> Html {
    let route = use_route().expect("called get_dashboard_tab with unknown route");
    match route {
        Route::DashboardHome => html! { <super::home::DashboardHome /> },
        Route::DashboardSleepHistory => html! { <super::sleep_history::SleepHistory /> },
        Route::Dashboard2 => html! { <h1>{"Tab 2"}</h1> },
        Route::DashboardProfile => html! { <super::profile_page::DashboardProfile /> },
        _ => panic!("called get_dashboard_tab with route that is not a dashboard tab (specifically, {route:?})"),
    }
}

#[function_component(DashboardTabColumn)]
pub fn dashboard_tab_column() -> Html {
    let cur_route: Route = use_route().expect("called dashboard_tab_column with unknown route");

    let routes = vec![
        (Route::DashboardHome, "Home"),
        (Route::DashboardSleepHistory, "Sleep history"),
        (Route::Dashboard2, "Tab 2"),
        (Route::DashboardProfile, "Profile Settings"),
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

    tabs
}
