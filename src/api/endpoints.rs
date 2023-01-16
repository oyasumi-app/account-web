use crate::api::*;
/// This contains macro-generated functions representing the different API requests that can be made.
use crate::endpoint;
use api_types::v1::*;

/// This struct is used to convince the type system to allow us to send_request a None body:
/// if it were Some, it would have to be a type that implements Serialize.
#[derive(serde::Serialize)]
struct NoBody;

/// This macro generates a function that sends a request to the API.
/// It accepts the following arguments:
///
/// - `name`: the name of the function, e.g. "auth_login"
/// - `path`: the path of the endpoint, e.g. "auth/login"
/// - `method`: the HTTP method to use, e.g. "POST"
/// - `body`: the type of the request body, e.g. `LoginRequest` (optional)
/// - `response`: the type of the response body, e.g. `LoginResponse`
macro_rules! api_request {
    // Request has body and response type
    ($name:ident, $path:expr, $method:ident, $body:ty, $response:ty) => {
        pub async fn $name(body: $body) -> Result<$response, gloo_net::Error> {
            log::debug!("-> {}({body:?})", stringify!($name));
            let url = endpoint!($path);
            let response = send_request(&url, gloo_net::http::Method::$method, Some(body)).await?;
            let response = response.json::<$response>().await?;
            log::debug!("<- {response:?}");
            Ok(response)
        }
    };

    // Request has no body and a response type
    ($name:ident, $path:expr, $method:ident, $response:ty) => {
        pub async fn $name() -> Result<$response, gloo_net::Error> {
            log::debug!("-> {}()", stringify!($name));
            let url = endpoint!($path);
            let missing_body: Option<NoBody> = None;
            let response =
                send_request(&url, gloo_net::http::Method::$method, missing_body).await?;
            let response = response.json::<$response>().await?;
            log::debug!("<- {response:?}");
            Ok(response)
        }
    };

    // Request has no body and no response type (returns whether request returned a 2xx status code)
    ($name:ident, $path:expr, $method:ident) => {
        pub async fn $name() -> Result<bool, gloo_net::Error> {
            log::debug!("-> {}()", stringify!($name));
            let url = endpoint!($path);
            let missing_body: Option<NoBody> = None;
            let response =
                send_request(&url, gloo_net::http::Method::$method, missing_body).await?;
            log::debug!("<- {response:?}");
            Ok(response.ok())
        }
    };
}

api_request!(auth_check, "auth/check", GET, CheckResponse);
api_request!(auth_login, "auth/login", POST, LoginRequest, LoginResponse);
api_request!(auth_get_current_token, "auth/token/@me", GET, TokenData);
api_request!(
    auth_register,
    "auth/register",
    POST,
    RegistrationRequest,
    RegistrationResponse
);
api_request!(auth_logout, "auth/token/@me", DELETE);
