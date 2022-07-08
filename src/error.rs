#[derive(Debug)]
pub enum Error {
    AlreadySignedUp,
    WrongCredentialsError,
    NotAuthenticated,
    MissingRefreshToken,
    InternalError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AlreadySignedUp => write!(f, "User already signed up."),
            Error::WrongCredentialsError => write!(f, "Wrong credentials."),
            Error::NotAuthenticated => write!(f, "User is not authenticated."),
            Error::MissingRefreshToken => write!(f, "Refresh Token is missing"),
            Error::InternalError => write!(f, "GoTrue internal error"),
        }
    }
}
