use crate::{
    api::{Api, EmailOrPhone},
    error::Error,
    session::Session,
    user_attributes::UserAttributes,
    user_update::UserUpdate,
};

#[derive(Debug, Clone)]
pub struct Client {
    current_session: Option<Session>,
    api: Api,
}

impl Client {
    /// Creates a GoTrue Client.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::Client;
    ///
    /// let client = Client::new("http://your.gotrue.endpoint".to_string());
    /// ```
    pub fn new(url: String) -> Client {
        Client {
            current_session: None,
            api: Api::new(url),
        }
    }

    /// Signs up a new user.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string();
    ///     let password = "some_password".to_string();
    ///     let res = client
    ///         .sign_up(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn sign_up(
        &mut self,
        email_or_phone: EmailOrPhone,
        password: &String,
    ) -> Result<Session, Error> {
        self.current_session = None;
        let result = self.api.sign_up(email_or_phone, password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                Ok(session)
            }
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::AlreadySignedUp);
                }
                Err(Error::InternalError)
            }
        }
    }

    /// Signs in a user.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string();
    ///     let password = "some_password".to_string();
    ///     let res = client
    ///         .sign_in(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn sign_in(
        &mut self,
        email_or_phone: EmailOrPhone,
        password: &String,
    ) -> Result<Session, Error> {
        self.current_session = None;
        let result = self.api.sign_in(email_or_phone, password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                Ok(session)
            }
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    Err(Error::WrongCredentials)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    /// Sends an OTP
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string();
    ///
    ///     let res = client
    ///         .send_otp(EmailOrPhone::Email(email), None)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn send_otp(
        &self,
        email_or_phone: EmailOrPhone,
        should_create_user: Option<bool>,
    ) -> Result<bool, Error> {
        let result = self.api.send_otp(email_or_phone, should_create_user).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "422" {
                    Err(Error::UserNotFound)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    pub async fn verify_otp<T: serde::Serialize>(&mut self, params: T) -> Result<bool, Error> {
        self.current_session = None;
        let result = self.api.verify_otp(params).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    Err(Error::WrongToken)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    /// Sign out the current user
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///
    ///     // Sign in first
    ///
    ///     let res = client.sign_out().await?;
    ///     Ok(())
    /// }
    pub async fn sign_out(&self) -> Result<bool, Error> {
        let result = match &self.current_session {
            Some(session) => self.api.sign_out(&session.access_token).await,
            None => return Err(Error::NotAuthenticated),
        };

        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Reset a user's password for an email address
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string()
    ///
    ///     let res = client.reset_password_for_email(&email).await?;
    ///     Ok(())
    /// }
    pub async fn reset_password_for_email(&self, email: &str) -> Result<bool, Error> {
        let result = self.api.reset_password_for_email(email).await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::UserNotFound),
        }
    }

    pub async fn update_user(&self, user: UserAttributes) -> Result<UserUpdate, Error> {
        let session = match &self.current_session {
            Some(s) => s,
            None => return Err(Error::NotAuthenticated),
        };

        let result = self.api.update_user(user, &session.access_token).await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    Err(Error::UserNotFound)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    /// Refreshes the current session
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///
    ///     // sign in first
    ///
    ///     client.refresh_session().await?:
    ///     Ok(())
    /// }
    pub async fn refresh_session(&mut self) -> Result<Session, Error> {
        if self.current_session.is_none() {
            return Err(Error::NotAuthenticated);
        }

        let result = match &self.current_session {
            Some(session) => self.api.refresh_access_token(&session.refresh_token).await,
            None => return Err(Error::MissingRefreshToken),
        };

        let session = match result {
            Ok(session) => session,
            Err(_) => return Err(Error::InternalError),
        };

        self.current_session = Some(session.clone());

        Ok(session)
    }

    /// Sets a session by refresh token
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let token = "refresh_token".to_string();
    ///
    ///     let session = client.set_session(token).await?:
    ///     Ok(())
    /// }
    pub async fn set_session(&mut self, refresh_token: &str) -> Result<Session, Error> {
        if refresh_token.is_empty() {
            return Err(Error::NotAuthenticated);
        }

        let result = self.api.refresh_access_token(refresh_token).await;

        let session = match result {
            Ok(session) => session,
            Err(_) => return Err(Error::InternalError),
        };

        self.current_session = Some(session.clone());

        Ok(session)
    }
}
