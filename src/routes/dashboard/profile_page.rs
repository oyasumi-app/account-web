use yew::prelude::*;

use crate::components::SessionList;

#[function_component(DashboardProfile)]
pub fn dashboard_profile() -> Html {
    html! {
        <>
            <div>
                <h1>{"Profile"}</h1>
            </div>
            <div class="row">
                <div class="col">
                    <SessionList />
                </div>
            </div>
        </>
    }
}
