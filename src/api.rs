pub mod request;
pub use request::*;
pub mod endpoints;
pub use endpoints::*;

pub const API_URL: &str = "http://localhost:3000/v1/";

/// A macro to construct API endpoint addresses
///
/// Example:
/// ```
/// let url = endpoint!("auth/login");
/// assert_eq!(url, "http://localhost:3000/v1/auth/login");
/// ```
#[macro_export]
macro_rules! endpoint {
    ($path:expr) => {
        &format!("{}{}", API_URL, $path)
    };
}
