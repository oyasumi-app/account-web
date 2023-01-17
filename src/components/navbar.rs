use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

mod user_menu;
use user_menu::UserMenu;

/// Creates a layout for generic dashboard pages.
/// Features a navbar at the top, which contains the logo and the user menu.
#[function_component(DashboardNavbar)]
pub fn dashboard_navbar() -> Html {
    html! {
    <nav class="navbar navbar-expand-lg bg-body-tertiary">
        <div class="container-fluid">
            <Link<Route> to={Route::DashboardHome} classes="navbar-brand">{"Navbar"}</Link<Route>>

        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#dashboardNavbarItemContainer" aria-controls="dashboardNavbarItemContainer" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="dashboardNavbarItemContainer">
            <ul class="navbar-nav">
                <li class="nav-item">
                    <Link<Route> to={Route::DashboardHome} classes="nav-link">{"Home"}</Link<Route>>
                </li>
            </ul>

            <ul class="navbar-nav ms-auto">
                <li class="nav-item">
                    <UserMenu />
                </li>
            </ul>

            </div>


        </div>
    </nav>
    }
}
