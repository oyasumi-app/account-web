use std::rc::Rc;

use api_types::Snowflake;
use yew::{prelude::*, suspense::use_future_with_deps};
use yew_bootstrap::{component::Alert, util::Color};
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::{
    api::*,
    components::{LoadingSpinner, Size},
    context::UserContext,
    Route,
};

#[function_component(SessionList)]
pub fn session_list() -> Html {
    let fallback = html! {
        <>
            <div class="btn-group mb-3" role="group">
                <button class="btn btn-primary placeholder" disabled={true}>{"Refresh"}</button>
                <button class="btn btn-danger placeholder" disabled={true}>{"Revoke all except current session"}</button>
            </div>
            <SessionListFallback />
            <SessionListFallback />
            <SessionListFallback />
        </>
    };

    html! {
        <>
        <h2>{ "Current active sessions" }</h2>
        <p>
            { "If you see a session that you don't recognize, you can revoke it here." }
        </p>
        <Suspense {fallback}>
            <SessionListInner />
        </Suspense>
        </>
    }
}

#[function_component(SessionListInner)]
fn session_list_inner() -> HtmlResult {
    let refresh_pulse = use_state(|| false);
    let refresh_pulse_out = refresh_pulse.clone();

    let sessions = use_future_with_deps(
        |_refresh_pulse_state| async {
            let res = auth_get_tokens().await;
            res
        },
        refresh_pulse_out,
    );
    let sessions = sessions?;

    let revoke_many = {
        let refresh_pulse_out = refresh_pulse.clone();
        use_async(async move {
            let res = auth_delete_other_tokens().await;
            if let Err(e) = res {
                log::error!("Failed to revoke many: {:?}", e);
            };
            refresh_pulse_out.set(!*refresh_pulse_out);
            Ok::<(), ()>(())
        })
    };

    let result_html = match &*sessions {
        Ok(ResponseType_auth_get_tokens::Status200(token_snowflakes)) => {
            let session_list_rows = token_snowflakes
                .iter()
                .map(|token_snowflake| {
                    html! {
                        <SessionListRow session_id={*token_snowflake} />
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
                        <button class="btn btn-danger" onclick={Callback::from(move |_| {
                            revoke_many.run();
                        })}>
                            { "Revoke all except current session" }
                        </button>
                    </div>


                    { session_list_rows }
                </>
            }
        }
        _ => {
            html! {
                <Alert style={Color::Danger}>{"Failed to load your sessions. Try reloading the page."}</Alert>
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
    let current_session = use_context::<Rc<UserContext>>()
        .expect("UserContext not found while rendering SessionListRow");
    let session = use_future_with_deps(
        |session_id| async move {
            let res = auth_get_token(*session_id).await;
            res
        },
        props.session_id,
    );
    let session = session?;

    let navigator = use_navigator().expect("Navigator not found while rendering SessionListRow");

    let token_id = use_state(|| props.session_id);
    let is_deleting = use_state(|| false);
    let is_deleting_into_callback = is_deleting.clone();
    let is_hidden = use_state(|| false);
    let is_hidden_into_callback = is_hidden.clone();

    let is_current_session = use_state(|| match &*current_session {
        UserContext::LoggedOut => panic!("Logged out while rendering SessionListRow"),
        UserContext::LoggedIn(token_data) => token_data.token.id == props.session_id,
    });
    let is_current_session_out = is_current_session.clone();

    let perform_revocation = {
        let is_current_session = is_current_session_out;
        let is_deleting = is_deleting_into_callback;
        let is_hidden = is_hidden_into_callback;
        use_async(async move {
            is_deleting.set(true);
            let res = auth_delete_token(*token_id).await;
            if let Ok(ResponseType_auth_delete_token::Status204(_)) = res {
                log::info!("Revoked token: {}", *token_id);
                is_hidden.set(true);
                if *is_current_session {
                    navigator.push(&Route::Login);
                }
            } else {
                log::error!("Failed to revoke token: {}", *token_id);
            }
            is_deleting.set(false);
            Ok::<(), ()>(())
        })
    };

    if *is_hidden {
        return Ok(html! {});
    }

    let highlight_class = match *is_current_session {
        true => Some("border-warning text-warning"),
        false => None,
    };
    let button_name = match *is_current_session {
        true => "Revoke (and log out)",
        false => "Revoke",
    };
    let button_class = match *is_current_session {
        true => "btn-danger",
        false => "btn-secondary",
    };
    let result_html = match &*session {
        Ok(ResponseType_auth_get_token::Status200(token_info)) => {
            html! {
                <div class={classes!("card", "mb-3", highlight_class)}>
                    <div class="card-body">
                        <h5 class="card-title">{token_info.token.id}</h5>
                        <p class="card-text">{format!("Expires at: {}", token_info.token.expires)}</p>
                        <button class={classes!("btn", button_class)} onclick={Callback::from(move |_| { perform_revocation.run(); })}>
                            <LoadingSpinner show={*is_deleting} size={Size::Small} />
                            {button_name}
                        </button>
                    </div>
                </div>
            }
        }
        _ => {
            html! {
                <Alert style={Color::Danger}>{"Failed to load info on session "}{props.session_id}{". Try reloading the page."}</Alert>
            }
        }
    };

    Ok(result_html)
}

#[function_component(SessionListFallback)]
fn session_list_fallback() -> Html {
    html! {
            <div class="card mb-3">
                <div class="card-body">
                    <h5 class="card-title" aria-hidden="true">
                        <span class="placeholder col-3"></span>
                        <LoadingSpinner show={true} size={Size::Small} />
                    </h5>
                    <p class="card-text">
                        <span class="placeholder col-6"></span>
                    </p>
                    <button class="btn btn-danger placeholder col-2">
                    </button>
                </div>
            </div>
    }
}
