use yew::prelude::*;
use yew_router::prelude::{use_route, Link};

use crate::Route;

#[function_component(DashboardTabContent)]
pub fn get_dashboard_tab() -> Html {
    let route = use_route().expect("called get_dashboard_tab with unknown route");
    match route {
        Route::DashboardHome => html! { <h1>{"Home"}</h1> },
        Route::Dashboard1 => html! { <h1>{"Tab 1"}</h1> },
        Route::Dashboard2 => html! { <h1>{"Tab 2"}</h1> },
        Route::Dashboard3 => html! { <h1>{"Tab 3"}</h1> },
        _ => panic!("called get_dashboard_tab with route that is not a dashboard tab (specifically, {route:?})"),
    }
}

#[function_component(DashboardTabColumn)]
pub fn dashboard_tab_column() -> Html {
    let cur_route: Route = use_route().expect("called dashboard_tab_column with unknown route");

    let routes = vec![
        (Route::DashboardHome, "Home"),
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

    tabs
}
