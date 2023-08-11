use yew::prelude::*;

use crate::components::SleepList;

#[function_component(SleepHistory)]
pub fn sleep_history() -> Html {
    html! {
        <>
            <div>
                <h1>{"Sleep History"}</h1>
            </div>
            <div class="row">
                <div class="col">
                    <SleepList />
                </div>
            </div>
        </>
    }
}
