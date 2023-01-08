use yew::prelude::*;
use yew_router::prelude::*;

mod check_login_required;
use check_login_required::CheckLoginRequired;
mod login;
use login::Login;
mod dashboard;
use dashboard::Dashboard;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,

    #[at("/login")]
    Login,
    #[at("/dashboard")]
    Dashboard,

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
        Route::Dashboard => html! { <Dashboard /> },

        Route::NotFound => html! { <h1>{ "404" }</h1> },
        #[allow(unreachable_patterns)]
        _ => html! { <h1>{ "Unknown route: this is a bug!" }</h1> },
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
