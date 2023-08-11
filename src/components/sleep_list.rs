use std::{future::Future, pin::Pin};

use api_types::Snowflake;
use yew::{prelude::*, suspense::use_future_with_deps};

use crate::{
    api::*,
    components::{AsyncButton, Color, DangerAlert, SleepTimer},
};

#[function_component(SleepList)]
pub fn sleep_list() -> Html {
    let fallback = html! {
        <>
            <div class="btn-group mb-3" role="group">
                <button class="btn btn-primary placeholder" disabled={true}>{"Refresh"}</button>
            </div>
            <SleepListFallback />
            <SleepListFallback />
            <SleepListFallback />
        </>
    };

    html! {
        <>
        <h2>{ "Sleep records" }</h2>
        <Suspense {fallback}>
            <SleepListInner />
        </Suspense>
        </>
    }
}

#[function_component(SleepListInner)]
fn sleep_list_inner() -> HtmlResult {
    let refresh_pulse = use_state(|| false);
    let refresh_pulse_out = refresh_pulse.clone();

    let states = use_future_with_deps(
        |_refresh_pulse_state| async {
            let res = sleep_get_list().await;
            res
        },
        refresh_pulse_out,
    );
    let states = states?;

    let result_html = match &*states {
        Ok(ResponseType_sleep_get_list::Status200(sleep_states)) => {
            let sleep_rows = sleep_states
                .iter()
                .map(|state| {
                    html! {
                        <SleepListRow sleep_id={state.id} />
                    }
                })
                .collect::<Html>();

            html! {
                <>
                    <div class="btn-group mb-3" role="group">
                        <button class="btn btn-primary" onclick={Callback::from(move |_| {
                            refresh_pulse.set(!*refresh_pulse);
                        })}>
                            { "Refresh" }
                        </button>
                    </div>


                    { sleep_rows }
                </>
            }
        }
        _ => {
            html! {
                <DangerAlert message="Failed to load your sleep records. Try reloading the page." />
            }
        }
    };

    Ok(result_html)
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct SleepListRowProps {
    sleep_id: Snowflake,
}

#[function_component(SleepListRow)]
fn sleep_list_row(props: &SleepListRowProps) -> HtmlResult {
    let sleep = use_future_with_deps(
        move |id| async move {
            let res = sleep_get_by_id(*id).await;
            res
        },
        props.sleep_id,
    );
    let sleep = sleep?;

    let token_id = use_state(|| props.sleep_id);
    let is_hidden = use_state(|| false);

    let delete_fn = {
        let is_hidden = is_hidden.clone();
        let token_id = token_id.clone();
        Callback::from(move |_: ()| -> Pin<Box<dyn Future<Output = ()>>> {
            let is_hidden = is_hidden.clone();
            let token_id = token_id.clone();
            Box::pin(async move {
                let outcome = match sleep_delete_by_id(*token_id).await {
                    Ok(v) => match v {
                        ResponseType_sleep_delete_by_id::Status204(_) => true,
                        ResponseType_sleep_delete_by_id::Status404(_) => true, // If it is already missing, it's deleted
                    },
                    Err(_) => false,
                };
                is_hidden.set(outcome);
            })
        })
    };

    if *is_hidden {
        return Ok(html!());
    }

    let result_html = match &*sleep {
        Ok(ResponseType_sleep_get_by_id::Status200(state)) => {
            let duration = match state.end {
                Some(end) => {
                    let duration = end - state.start;
                    let seconds = duration.num_seconds();
                    let minutes = seconds / 60;
                    let hours = minutes / 60;
                    let minutes = minutes % 60;
                    let seconds = seconds % 60;

                    format!("{hours}:{minutes:02}:{seconds:02}")
                }
                None => String::new(),
            };

            let end_lines = match state.end {
                Some(end) => html!(<>
                    <p class="card-text">{"Ended at: "}{end}</p>
                    <p class="card-text">{"Elapsed: "}{duration}</p>
                </>),
                None => html!(
                    <>
                        <p class="card-text">{"Not ended yet "}</p>
                        <p class="card-text">{"Elapsed: "}<SleepTimer since={state.start} /></p>
                    </>
                ),
            };
            html! {
                <div class={classes!("card", "mb-3")}>
                    <div class="card-body">
                        <h5 class="card-title">{state.id}</h5>
                        <p class="card-text">{"Started at: "}{state.start}</p>
                        {end_lines}
                        <div class="btn-group">
                            <AsyncButton class="" text="Delete" color={Color::Danger} onclick_fn={delete_fn} />
                        </div>
                    </div>
                </div>
            }
        }
        _ => {
            html! {
                <DangerAlert message={format!("Failed to load info on sleep state {}. Try reloading the page.", props.sleep_id)} />
            }
        }
    };

    Ok(result_html)
}

#[function_component(SleepListFallback)]
fn sleep_list_fallback() -> Html {
    let delete_fn = {
        Callback::from(move |_: ()| -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async move { () })
        })
    };
    html! {
        <div class={classes!("card", "mb-3")}>
            <div class="card-body">
                <h5 class="card-title"><span class="placeholder col-5" /></h5>
                <p class="card-text"><span class="placeholder col-4" /></p>
                <p class="card-text"><span class="placeholder col-4" /></p>
                <p class="card-text"><span class="placeholder col-3" /></p>
                <div class="btn-group">
                    <AsyncButton enabled={false} class="" text="Delete" color={Color::Danger} onclick_fn={delete_fn} />
                </div>
            </div>
        </div>
    }
}