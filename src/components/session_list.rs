use api_types::Snowflake;
use yew::{prelude::*, suspense::use_future_with_deps, platform::spawn_local};

use crate::{components::{ModalLoadingSpinner, DangerAlert, LoadingSpinner, Size}, api::*};

#[function_component(SessionList)]
pub fn session_list() -> Html {
    let fallback = html! {
        <ModalLoadingSpinner text="Loading your sessions..." />
    };

    html! {
        <Suspense {fallback}>
            <SessionListInner />
        </Suspense>
    }
}

#[function_component(SessionListInner)]
fn session_list_inner() -> HtmlResult {
    let refresh_pulse = use_state(|| false);
    let refresh_pulse_out = refresh_pulse.clone();

    let sessions = use_future_with_deps(|_refresh_pulse_state| async {
        let res = auth_get_tokens().await;
        res
    }, refresh_pulse_out);
    let sessions = sessions?;

    let result_html = match &*sessions {
        Ok(token_snowflakes) => {
            let session_list_rows = token_snowflakes.iter().map(|token_snowflake| {
                html! {
                    <SessionListRow session_id={*token_snowflake} />
                }
            }).collect::<Html>();

            html!{
                <>
                    <h2>{ "Current active sessions" }</h2>
                    <p>
                        { "If you see a session that you don't recognize, you can revoke it here." }
                    </p>
                    <button class="btn btn-primary" onclick={Callback::from(move |_| {
                        refresh_pulse.set(!*refresh_pulse);
                    })}>
                        { "Refresh" }
                    </button>
                    { session_list_rows }
                </>
            }

        },
        Err(_) => {
            html! {
                <DangerAlert message="Failed to load your sessions. Try reloading the page." />
            }
        }
    };

    Ok(result_html)
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct SessionListRowProps {
    session_id: Snowflake,
}

#[function_component(SessionListRow)]
fn session_list_row(props: &SessionListRowProps) -> HtmlResult {
    let session = use_future_with_deps(|session_id| async move {
        let res = auth_get_token(*session_id).await;
        res
    }, props.session_id);
    let session = session?;

    let token_id = use_state(|| props.session_id);
    let is_deleting = use_state(|| false);
    let is_deleting_into_callback = is_deleting.clone();
    let is_hidden = use_state(|| false);
    let is_hidden_into_callback = is_hidden.clone();

    let perform_revocation = Callback::from(move |_| {
        let token_id = token_id.clone();
        let is_deleting = is_deleting_into_callback.clone();
        let is_hidden = is_hidden_into_callback.clone();
        spawn_local(async move {
            is_deleting.set(true);
            let res = auth_delete_token(*token_id).await;
            if let Ok(true) = res {
                log::info!("Revoked token: {}", *token_id);
                is_hidden.set(true);
            } else {
                log::error!("Failed to revoke token: {}", *token_id);
            }
            is_deleting.set(false);
        });
    });

    if *is_hidden {
        return Ok(html!{});
    }

    let result_html = match &*session {
        Ok(token_info) => {
            html! {
                <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">{token_info.token.id}</h5>
                        <p class="card-text">{format!("Expires at: {}", token_info.token.expires)}</p>
                        <button class="btn btn-danger" onclick={perform_revocation}>
                            <LoadingSpinner show={*is_deleting} size={Size::Small} />
                            {"Revoke"}
                        </button>
                    </div>
                </div>
            }
        },
        Err(_) => {
            html! {
                <DangerAlert message={format!("Failed to load info on session {}. Try reloading the page.", props.session_id)} />
            }
        }
    };

    Ok(result_html)
}