#[derive(Debug)]
pub enum Error {
    AlreadySignedUp,
    WrongCredentials,
    UserNotFound,
    NotAuthenticated,
    MissingRefreshToken,
    WrongToken,
    InternalError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AlreadySignedUp => write!(f, "User already signed up."),
            Error::WrongCredentials => write!(f, "Wrong credentials."),
            Error::UserNotFound => write!(f, "User not found."),
            Error::NotAuthenticated => write!(f, "User is not authenticated."),
            Error::MissingRefreshToken => write!(f, "Refresh Token is missing"),
            Error::WrongToken => write!(f, "Wrong token."),
            Error::InternalError => write!(f, "GoTrue internal error"),
        }
    }
}
