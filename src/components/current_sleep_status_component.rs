use std::time::{Duration, SystemTime};

use api_types::v1::DateTimeUtc;
use yew::{prelude::*, suspense::use_future_with_deps};
use yew_hooks::{use_async, use_interval, use_update};

use crate::{
    api::{
        sleep_create_new_current, sleep_delete_current, sleep_get_current, sleep_set_current_end,
        sleep_set_current_start,
    },
    components::{Button, Color},
};

/// Component that indicates the current sleep state
/// and allows changing it.
#[function_component(CurrentSleepStatusComponent)]
pub fn current_sleep_status_component() -> Html {
    let fallback = html!(
        <div class="card">
            <div class="card-body">
                <h4 class="card-title">{"Loading current sleep state..."}</h4>
                <p class="card-text placeholder-wave">
                    <span class="placeholder col-7" />
                </p>
            </div>
        </div>
    );

    html!(
        <Suspense {fallback}>
            <CurrentSleepStatusInner />
        </Suspense>
    )
}
#[function_component(CurrentSleepStatusInner)]
fn current_sleep_status_inner() -> HtmlResult {
    let deps = use_state(|| 0usize);
    let sleep_state = use_future_with_deps(|_| async move { sleep_get_current().await }, *deps);
    let sleep_state = sleep_state?;

    let wake_up_action = {
        let deps = deps.clone();
        use_async(async move {
            let result = sleep_set_current_end().await.ok().ok_or(());
            deps.set(*deps + 1);
            result
        })
    };

    let wake_up_cb = {
        let wake_up_action = wake_up_action.clone();
        Callback::from(move |_: ()| {
            wake_up_action.run();
        })
    };

    let start_sleep_action = {
        let deps = deps.clone();
        use_async(async move {
            let result = sleep_create_new_current().await.ok().ok_or(());
            deps.set(*deps + 1);
            result
        })
    };

    let start_sleep_cb = {
        let start_sleep_action = start_sleep_action.clone();
        Callback::from(move |_: ()| {
            start_sleep_action.run();
        })
    };

    let update_start_action = {
        let deps = deps.clone();
        use_async(async move {
            let result = sleep_set_current_start().await.ok().ok_or(());
            deps.set(*deps + 1);
            result
        })
    };

    let update_start_cb = {
        let update_start_action = update_start_action.clone();
        Callback::from(move |_: ()| {
            update_start_action.run();
        })
    };

    let delete_sleep_action = {
        let deps = deps.clone();
        use_async(async move {
            let result = sleep_delete_current().await.ok().ok_or(());
            deps.set(*deps + 1);
            result
        })
    };

    let delete_sleep_cb = {
        let delete_sleep_action = delete_sleep_action.clone();
        Callback::from(move |_: ()| {
            delete_sleep_action.run();
        })
    };

    let res = match &*sleep_state {
        Ok(state) => match state {
            crate::api::ResponseType_sleep_get_current::Status200(state) => {
                html!(
                    <div class="card">
                        <div class="card-body">
                            <h4 class="card-title">{"You have been sleeping for "}<SleepTimer since={state.start} /></h4>

                            <Button class="btn btn-success w-100 btn-lg mb-2" text="Wake up" color={Color::Success} onclick={wake_up_cb} spinning={wake_up_action.loading}/>
                            <div class="btn-group btn-sm w-100 mb-2">
                                <Button class="" text="Could not fall asleep before" color={Color::Warning} onclick={update_start_cb} spinning={update_start_action.loading}/>
                                <Button class="" text="Did not go to sleep at all" color={Color::Danger} onclick={delete_sleep_cb} spinning={delete_sleep_action.loading}/>
                            </div>

                        </div>
                    </div>
                )
            }
            crate::api::ResponseType_sleep_get_current::Status404(_) => html!(
                <div class="card">
                    <div class="card-body">
                        <h4 class="card-title">{"You are not currently sleeping"}</h4>

                        <Button class="btn w-100 btn-lg mb-2" text="Go to sleep" color={Color::Primary} onclick={start_sleep_cb} spinning={start_sleep_action.loading}/>
                    </div>
                </div>
            ),
        },
        Err(err) => {
            html!(<div class="alert alert-danger">{"Failed to get current sleep state because: "}{err}</div>)
        }
    };

    Ok(res)
}

#[derive(PartialEq, Debug, Clone, Properties)]
struct SleepTimerProps {
    since: DateTimeUtc,
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    pub fn get_unix_timestamp() -> f64;
}

#[function_component(SleepTimer)]
fn sleep_timer(props: &SleepTimerProps) -> Html {
    let update = use_update();
    use_interval(move || update(), 1000);

    let now = unsafe { get_unix_timestamp() };
    let now = DateTimeUtc::from(SystemTime::UNIX_EPOCH + Duration::from_secs_f64(now));

    let elapsed = props.since - now;

    html!(<>{elapsed}</>)
}
