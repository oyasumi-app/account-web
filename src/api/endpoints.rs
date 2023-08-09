use crate::api::*;
/// This contains macro-generated functions representing the different API requests that can be made.
use crate::endpoint;
use api_types::v1::*;
use api_types::Snowflake;
use gloo_net::http::Response;
use serde::de::DeserializeOwned;

/// This struct is used to convince the type system to allow us to send_request a None body:
/// if it were Some, it would have to be a type that implements Serialize.
#[derive(serde::Serialize)]
struct NoBody;

/// This trait is implemented on data types which can be passed as path arguments.
trait AsPathFragment {
    fn to_path_fragment(&self) -> String;
}

impl AsPathFragment for Snowflake {
    fn to_path_fragment(&self) -> String {
        self.to_string()
    }
}

/// This function parses a [`gloo_net::http::Response`] with JSON,
/// except for if we want to parse into the unit type;
/// in that case, it just returns the unit type.
async fn parse_json<T: DeserializeOwned>(response: Response) -> Result<T, gloo_net::Error> {
    // If the type's size is equal to zero, it is the unit type.
    // So do not try deserializing it.
    if std::mem::size_of::<T>() == 0 {
        // Instead, conjure an instance of the ZST by transmuting the unit type
        // into the given type.
        // This is safe, because we know that both types have the same size (zero).
        let output: T = unsafe { std::mem::transmute_copy(&()) };
        return Ok(output);
    }

    response.json::<T>().await
}

macro_rules! api_request {
    // Request has no body: api_request!(check_token: GET "auth/token/@me" => (200 TokenData) (404 ()) )
    ($name:ident : $method:ident $path:literal => $( ( $status:literal $responsetype:ty ) )+) => {
        paste::paste!{
            #[derive(Debug)]
            #[allow(non_camel_case_types)]
            pub enum [<ResponseType_ $name>] {
                $(
                    [<Status $status>]($responsetype),
                )*
            }
            pub async fn $name() -> Result<[<ResponseType_ $name>], gloo_net::Error> {
                log::debug!("-> {}()", stringify!($name));
                let url = endpoint!($path);
                let missing_body: Option<NoBody> = None;
                let response =
                    send_request(&url, gloo_net::http::Method::$method, missing_body).await?;
                let response = match response.status() {
                    $(
                        $status => {
                            let content = response.json::<$responsetype>().await?;
                            [<ResponseType_ $name>]::[<Status $status>](content)
                        },
                    )*
                    other => return Err(gloo_net::Error::GlooError(format!("In $name, received unexpected status: {}", other)))
                };
                log::debug!("<- {response:?}");
                Ok(response)
            }
        }
    };

    // Request has a body: api_request!(check_token: POST "auth/login" (LoginRequest) => (200 LoginResponse) (404 ()) )
    ($name:ident : $method:ident $path:literal ($requestbody:ty) => $( ( $status:literal $responsetype:ty ) )+) => {
        paste::paste!{
            #[derive(Debug)]
            #[allow(non_camel_case_types)]
            pub enum [<ResponseType_ $name>] {
                $(
                    [<Status $status>]($responsetype),
                )*
            }
            pub async fn $name(body: $requestbody) -> Result<[<ResponseType_ $name>], gloo_net::Error> {
                log::debug!("-> {}()", stringify!($name));
                let url = endpoint!($path);
                let body = Some(body);
                let response =
                    send_request(&url, gloo_net::http::Method::$method, body).await?;
                let response = match response.status() {
                    $(
                        $status => {
                            let content = response.json::<$responsetype>().await?;
                            [<ResponseType_ $name>]::[<Status $status>](content)
                        },
                    )*
                    other => return Err(gloo_net::Error::GlooError(format!("In $name, received unexpected status: {}", other)))
                };
                log::debug!("<- {response:?}");
                Ok(response)
            }
        }
    };
}

macro_rules! ident_as_format_question {
    ($what:ident) => {
        "{:?}, "
    };
}

macro_rules! api_request_with_path {
    // Request has no body: api_request!(registration_get: GET "auth/registration/{}" (reg_id Snowflake, .to_string()) => (200 TokenData) (404 ()) )
    ($name:ident : $method:ident $path_format_string:literal $( ( $path_fragment_name: ident $path_fragment_type:ty $(,)? ) )* => $( ( $status:literal $responsetype:ty ) )+) => {
        paste::paste!{
            #[derive(Debug)]
            #[allow(non_camel_case_types)]
            pub enum [<ResponseType_ $name>] {
                $(
                    [<Status $status>]($responsetype),
                )*
            }
            pub async fn $name(
                $(
                    $path_fragment_name : $path_fragment_type,
                )*

            ) -> Result<[<ResponseType_ $name>], gloo_net::Error> {
                log::debug!(
                    concat!("-> ", stringify!($name), "(",
                    $(
                        ident_as_format_question!($path_fragment_name),
                    )*
                    ")"
                    ),
                $($path_fragment_name,)*
                );
                let url_fragment = format!(
                    $path_format_string,
                    $(
                        AsPathFragment::to_path_fragment(&$path_fragment_name),
                    )*
                );
                let url = endpoint!(url_fragment);

                let missing_body: Option<NoBody> = None;
                let response =
                    send_request(&url, gloo_net::http::Method::$method, missing_body).await?;
                let response = match response.status() {
                    $(
                        $status => {
                            let content = parse_json::<$responsetype>(response).await?;
                            [<ResponseType_ $name>]::[<Status $status>](content)
                        },
                    )*
                    other => return Err(gloo_net::Error::GlooError(format!("In $name, received unexpected status: {}", other)))
                };
                log::debug!("<- {response:?}");
                Ok(response)
            }
        }
    };

    // Request has body: api_request!(registration_get: GET "auth/registration/{}" (reg_id Snowflake, .to_string()) => (200 TokenData) (404 ()) )
    ($name:ident : $method:ident $path_format_string:literal $( ( $path_fragment_name: ident $path_fragment_type:ty ) $(,)? )* => $body_type:ty => $( ( $status:literal $responsetype:ty ) )+) => {
        paste::paste!{
            #[derive(Debug)]
            #[allow(non_camel_case_types)]
            pub enum [<ResponseType_ $name>] {
                $(
                    [<Status $status>]($responsetype),
                )*
            }
            pub async fn $name(
                $(
                    $path_fragment_name : $path_fragment_type,
                )*
                body: $body_type,

            ) -> Result<[<ResponseType_ $name>], gloo_net::Error> {
                log::debug!(
                    concat!("-> ", stringify!($name), "(",
                    $(
                        ident_as_format_question!($path_fragment_name),
                    )*
                    "{:?})" // for body
                    ),
                $($path_fragment_name,)*
                body
                );
                let url_fragment = format!(
                    $path_format_string,
                    $(
                        AsPathFragment::to_path_fragment(&$path_fragment_name),
                    )*
                );
                let url = endpoint!(url_fragment);

                let body = Some(body);
                let response =
                    send_request(&url, gloo_net::http::Method::$method, body).await?;
                let response = match response.status() {
                    $(
                        $status => {
                            let content = parse_json::<$responsetype>(response).await?;
                            [<ResponseType_ $name>]::[<Status $status>](content)
                        },
                    )*
                    other => return Err(gloo_net::Error::GlooError(format!("In $name, received unexpected status: {}", other)))
                };
                log::debug!("<- {response:?}");
                Ok(response)
            }
        }
    };
}
api_request!(auth_check: GET "auth/check" => (200 CheckResponse));
api_request!(auth_login: POST "auth/login" (LoginRequest) => (200 LoginSuccess) (401 LoginError));
api_request!(auth_get_current_token: GET "auth/token/@me" => (200 TokenData));
api_request!(
    auth_register: POST
    "auth/registration" (RegistrationRequest) => (200 RegistrationResponse)
);
api_request!(auth_logout: DELETE "auth/token/@me" => (204 ()));
api_request!(auth_get_tokens: GET "auth/token/list" => (200 Vec<Snowflake>));

api_request_with_path!(auth_get_token: GET "auth/token/by_id/{}" (id Snowflake,) => (200 TokenData) (404 ()));
api_request_with_path!(auth_delete_token: DELETE "auth/token/by_id/{}" (id Snowflake,) => (204 ()) (404 ()));
api_request!(auth_delete_other_tokens: DELETE "auth/token/list" => (204 ()));

api_request_with_path!(registration_get: GET "auth/registration/{}" (id Snowflake,) => (200 PendingRegistration) (404 ()));
api_request_with_path!(registration_confirm: POST "auth/registration/{}/confirm" (id Snowflake) => ConfirmRegistrationRequest => (200 ConfirmRegistrationResponse));
