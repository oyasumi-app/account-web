use yew::{prelude::*, suspense::use_future_with_deps};

use crate::{api::events_get_stream_list, components::DangerAlert};

#[function_component(StreamList)]
pub fn stream_list() -> Html {
    let fallback = html! {
        <div>{"Loading..."}</div>
    };

    html! {
        <Suspense fallback={fallback}>
            <StreamListInner />
        </Suspense>
    }
}

#[function_component(StreamListInner)]
pub fn stream_list_inner() -> HtmlResult {
    let streams = use_future_with_deps(
        |_| async move {
            let res = events_get_stream_list().await;
            res
        },
        (),
    );
    let streams = streams?;
    if streams.is_err() {
        return Ok(html! {
            <DangerAlert message="Failed to load streams." />
        });
    }
    let streams = streams.as_ref().unwrap();

    let stream_components = streams
        .iter()
        .map(|stream| {
            html! {
                <div>
                    <h1>{&stream.id}</h1>
                    <p>{&stream.name}</p>
                </div>
            }
        })
        .collect::<Html>();

    Ok(stream_components)
}
