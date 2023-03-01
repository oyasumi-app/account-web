use api_types::Snowflake;
use yew::prelude::*;
use yew_router::prelude::*;

mod check_login_required;
use check_login_required::CheckLoginRequired;
mod login;
use login::Login;
mod dashboard;
use dashboard::Dashboard;
mod register;
use register::Register;
mod confirm_register;
use confirm_register::ConfirmRegister;

use crate::components::BigError;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,

    #[at("/login")]
    Login,
    #[at("/dashboard")]
    DashboardHome,

    #[at("/dashboard/1")]
    Dashboard1,
    #[at("/dashboard/2")]
    Dashboard2,
    #[at("/dashboard/profile")]
    DashboardProfile,

    #[at("/registration/new")]
    Register,
    #[at("/registration/:id/confirm")]
    ConfirmRegister { id: Snowflake },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <CheckLoginRequired /> },
        Route::Secure => html! {
            <Secure />
        },
        Route::Login => html! { <Login /> },
        Route::DashboardHome => html! { <Dashboard /> },

        Route::Dashboard1 => html! { <Dashboard /> },
        Route::Dashboard2 => html! { <Dashboard /> },
        Route::DashboardProfile => html! { <Dashboard /> },

        Route::Register => html! { <Register /> },
        Route::ConfirmRegister { id } => html! { <ConfirmRegister registration_id={id} /> },

        Route::NotFound => html! { <h1>{ "404" }</h1> },
        #[allow(unreachable_patterns)]
        route => html! {
            <BigError short_name="Unimplemented route"
                text="You visited an route that was recognized by the app, but which has not been implemented. This is an application bug."
                diagnostics={format!("attempted route {route:?} fell through routing match statement")}
            />
        },
    }
}

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
