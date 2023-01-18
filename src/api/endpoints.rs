use crate::api::*;
/// This contains macro-generated functions representing the different API requests that can be made.
use crate::endpoint;
use api_types::Snowflake;
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
/// - `response`: the type of the response body, e.g. `LoginResponse` (optional, if not provided, the function returns a bool of status==2xx)
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

/// This macro generates a function that sends a request to the API,
/// where the path contains placeholders that are replaced by the arguments.
/// It accepts the following arguments:
/// - `name`: the name of the function, e.g. "auth_get_token"
/// - `path`: the path of the endpoint as a format string, e.g. "auth/token/by_id/{args.0}"
/// - `method`: the HTTP method to use, e.g. "GET"
/// - `args`: the type of the arguments, e.g. `(Snowflake,)`
/// - `response`: the type of the response body, e.g. `TokenData`
macro_rules! api_request_with_path {
    // Request has a body and a response type
    ($name: ident, $method: ident, $arg_type: ty, $body_type: ty, $response: ty, $path_format_string: literal, $($path_format_arg: tt)*) => {
        pub async fn $name(args: $arg_type, body: $body_type) -> Result<$response, gloo_net::Error> {
            log::debug!("-> {}({args:?}; {body:?})", stringify!($name));
            let url_fragment = format!($path_format_string, args $($path_format_arg)*);
            let url = endpoint!(url_fragment);
            let body = Some(body);
            let response =
                send_request(&url, gloo_net::http::Method::$method, body).await?;
            let response = response.json::<$response>().await?;
            log::debug!("<- {response:?}");
            Ok(response)
        }
    };


    // Request has no body and a response type
    ($name: ident, $method: ident, $arg_type: ty, $response: ty, $path_format_string: literal, $($path_format_arg: tt)*) => {
        pub async fn $name(args: $arg_type) -> Result<$response, gloo_net::Error> {
            log::debug!("-> {}({args:?})", stringify!($name));
            let url_fragment = format!($path_format_string, args $($path_format_arg)*);
            let url = endpoint!(url_fragment);
            let missing_body: Option<NoBody> = None;
            let response =
                send_request(&url, gloo_net::http::Method::$method, missing_body).await?;
            let response = response.json::<$response>().await?;
            log::debug!("<- {response:?}");
            Ok(response)
        }
    };

    // Request has no body and no response type (returns whether request returned a 2xx status code)
    ($name: ident, $method: ident, $arg_type: ty, $path_format_string: literal, $($path_format_arg: tt)*) => {
        pub async fn $name(args: $arg_type) -> Result<bool, gloo_net::Error> {
            log::debug!("-> {}({args:?})", stringify!($name));
            let url_fragment = format!($path_format_string, args $($path_format_arg)*);
            let url = endpoint!(url_fragment);
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
api_request!(auth_get_tokens, "auth/token/list", GET, Vec<Snowflake>);

api_request_with_path!(auth_get_token, GET, Snowflake, TokenData, "auth/token/by_id/{}", .to_string());
api_request_with_path!(auth_delete_token, DELETE, Snowflake, "auth/token/by_id/{}", .to_string());
api_request!(auth_delete_other_tokens, "auth/token/list", DELETE);