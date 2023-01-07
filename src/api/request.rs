use serde::Serialize;

/// Module containing methods to send requests to the API.

/// Send a request to the API.
/// Attach the Authorization header if there is a token cookie available.
pub async fn send_request(
    url: &str,
    method: reqwest::Method,
    body: Option<impl Serialize>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut request = client.request(method, url);

    if let Some(body) = body {
        request = request.json(&body);
    }

    let response = request.send().await?;

    Ok(response)
}
