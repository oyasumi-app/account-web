use api_types::v1::TokenData;

#[derive(Clone, Debug, PartialEq)]
pub enum UserContext {
    LoggedIn(TokenData),
    LoggedOut,
}
