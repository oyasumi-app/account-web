use std::{
    future::Future,
    pin::Pin,
    time::{Duration, SystemTime},
};

use api_types::v1::DateTimeUtc;
use yew::{prelude::*, suspense::use_future_with_deps};
use yew_hooks::{use_interval, use_update};

use crate::{
    api::{
        sleep_create_new_current, sleep_delete_current, sleep_get_current, sleep_set_current_end,
        sleep_set_current_start,
    },
    components::{AsyncButton, Button, Color},
};

/// Component that indicates the current sleep state
/// and allows changing it.
#[function_component(CurrentSleepStatusComponent)]
pub fn current_sleep_status_component() -> Html {
    let fallback = html!(
        <div class="card">
            <div class="card-body">
                <h4 class="card-title">{"Loading current sleep state..."}</h4>
                <Button class="w-100 btn-lg mb-2" text="Loading..." color={Color::Primary} onclick={Callback::noop()} />
            </div>
        </div>
    );

    html!(
        <Suspense {fallback}>
            <CurrentSleepStatusInner />
        </Suspense>
    )
}

trait Ignorable {
    fn ignore(self);
}

impl<T, E> Ignorable for Result<T, E> {
    fn ignore(self) {}
}

#[function_component(CurrentSleepStatusInner)]
fn current_sleep_status_inner() -> HtmlResult {
    let deps = use_state(|| 0usize);
    let sleep_state = use_future_with_deps(|_| async move { sleep_get_current().await }, *deps);
    let sleep_state = sleep_state?;

    let wake_up_fn = {
        let deps = deps.clone();
        Callback::from(move |_: ()| -> Pin<Box<dyn Future<Output = ()>>> {
            let deps = deps.clone();
            Box::pin(async move {
                sleep_set_current_end().await.ignore();
                deps.set(*deps + 1);
            })
        })
    };

    let start_sleep_fn = {
        let deps = deps.clone();
        Callback::from(move |_: ()| -> Pin<Box<dyn Future<Output = ()>>> {
            let deps = deps.clone();
            Box::pin(async move {
                sleep_create_new_current().await.ignore();
                deps.set(*deps + 1);
            })
        })
    };

    let update_start_fn = {
        let deps = deps.clone();
        Callback::from(move |_: ()| -> Pin<Box<dyn Future<Output = ()>>> {
            let deps = deps.clone();
            Box::pin(async move {
                sleep_set_current_start().await.ignore();
                deps.set(*deps + 1);
            })
        })
    };

    let delete_sleep_fn = {
        let deps = deps.clone();
        Callback::from(move |_: ()| -> Pin<Box<dyn Future<Output = ()>>> {
            let deps = deps.clone();
            Box::pin(async move {
                sleep_delete_current().await.ignore();
                deps.set(*deps + 1);
            })
        })
    };

    let res = match &*sleep_state {
        Ok(state) => match state {
            crate::api::ResponseType_sleep_get_current::Status200(state) => {
                html!(
                    <div class="card">
                        <div class="card-body">
                            <h4 class="card-title">{"You have been sleeping for "}<SleepTimer since={state.start} /></h4>

                            <AsyncButton class="btn-success w-100 btn-lg mb-2" text="Wake up" color={Color::Success} onclick_fn={wake_up_fn} />
                            <div class="btn-group btn-sm w-100 mb-2">
                                <AsyncButton class="" text="Could not fall asleep before" color={Color::Warning} onclick_fn={update_start_fn} />
                                <AsyncButton class="" text="Did not go to sleep at all" color={Color::Danger} onclick_fn={delete_sleep_fn}/>
                            </div>

                        </div>
                    </div>
                )
            }
            crate::api::ResponseType_sleep_get_current::Status404(_) => html!(
                <div class="card">
                    <div class="card-body">
                        <h4 class="card-title">{"You are not currently sleeping"}</h4>

                        <AsyncButton class="w-100 btn-lg mb-2" text="Go to sleep" color={Color::Primary} onclick_fn={start_sleep_fn}/>
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

    #[allow(unused_unsafe)]
    let now = unsafe { get_unix_timestamp() };
    let now = DateTimeUtc::from(SystemTime::UNIX_EPOCH + Duration::from_secs_f64(now));

    let elapsed = now - props.since;
    let seconds = elapsed.num_seconds();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let minutes = minutes % 60;
    let seconds = seconds % 60;
    let time = format!("{hours}:{minutes:02}:{seconds:02}");

    html!(<>{time}</>)
}
