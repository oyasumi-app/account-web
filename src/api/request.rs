use gloo_net::http::*;
use serde::Serialize;
use gloo_net::http::Request;
use gloo_net::Error;

/// Module containing methods to send requests to the API.

/// Send a request to the API.
/// Attach the Authorization header if there is a token cookie available.
pub async fn send_request(
    url: &str,
    method: Method,
    body: Option<impl Serialize>,
) -> Result<Response, Error> {
    let request = Request::new(url)
            .method(method);
    let request_with_body;
    if let Some(body) = body {
        request_with_body = request.json(&body)?;
    } else {
        request_with_body = request;
    }
    let request = request_with_body.credentials(RequestCredentials::Include);
    request.send().await
}
