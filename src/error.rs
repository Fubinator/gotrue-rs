use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Use already signed up.")]
    AlreadySignedUp,
    #[error("Wrong credentials.")]
    WrongCredentials,
    #[error("User not found.")]
    UserNotFound,
    #[error("User is not authenticated.")]
    NotAuthenticated,
    #[error("Refresh Token is missing")]
    MissingRefreshToken,
    #[error("Wrong token.")]
    WrongToken,
    #[error("GoTrue internal error")]
    InternalError,
}
