use thiserror::Error;

///
/// Represents a error returned from a user action by GoTrue.
///
#[derive(Error, Debug)]
pub enum Error {
    /// The user has already signed up.
    #[error("Use already signed up.")]
    AlreadySignedUp,

    /// The user has provided the incorrect credentials.
    #[error("Wrong credentials.")]
    WrongCredentials,

    /// The user was not found.
    #[error("User not found.")]
    UserNotFound,

    /// The user could not be authenticated.
    #[error("User is not authenticated.")]
    NotAuthenticated,

    /// The required refresh token was missing.
    #[error("Refresh Token is missing")]
    MissingRefreshToken,

    /// The wrong token was provided.
    #[error("Wrong token.")]
    WrongToken,

    /// Internal GoTrue error.
    #[error("GoTrue internal error")]
    InternalError,
}
