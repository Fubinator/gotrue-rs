use crate::{
    api::{Api, EmailOrPhone},
    error::Error,
    session::Session,
    user_attributes::UserAttributes,
    user_update::UserUpdate,
};

pub struct Client {
    current_session: Option<Session>,
    api: Api,
}

impl Client {
    pub fn new(url: String) -> Client {
        Client {
            current_session: None,
            api: Api::new(url),
        }
    }

    pub async fn sign_up(&mut self, email: &String, password: &String) -> Result<Session, Error> {
        self.current_session = None;
        let result = self.api.sign_up(&email, &password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return Ok(session);
            }
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::AlreadySignedUp);
                }
                return Err(Error::InternalError);
            }
        }
    }

    pub async fn sign_in(&mut self, email: &String, password: &String) -> Result<Session, Error> {
        self.current_session = None;
        let result = self.api.sign_in(&email, &password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return Ok(session);
            }
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::WrongCredentials);
                }
                return Err(Error::InternalError);
            }
        }
    }

    pub async fn send_otp(
        &self,
        email_or_phone: EmailOrPhone,
        should_create_user: Option<bool>,
    ) -> Result<bool, Error> {
        let result = self.api.send_otp(email_or_phone, should_create_user).await;

        match result {
            Ok(_) => return Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "422" {
                    return Err(Error::UserNotFound);
                }
                return Err(Error::InternalError);
            }
        }
    }

    pub async fn verify_otp<T: serde::Serialize>(&mut self, params: T) -> Result<bool, Error> {
        self.current_session = None;
        let result = self.api.verify_otp(params).await;

        match result {
            Ok(_) => return Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::WrongToken);
                }
                return Err(Error::InternalError);
            }
        }
    }

    pub async fn sign_out(&self) -> Result<bool, Error> {
        let result = match &self.current_session {
            Some(session) => self.api.sign_out(&session.access_token).await,
            None => return Err(Error::NotAuthenticated),
        };

        match result {
            Ok(_) => return Ok(true),
            Err(_) => return Err(Error::InternalError),
        }
    }

    pub async fn reset_password_for_email(&self, email: &str) -> Result<bool, Error> {
        let result = self.api.reset_password_for_email(&email).await;

        match result {
            Ok(_) => return Ok(true),
            Err(_) => return Err(Error::UserNotFound),
        }
    }

    pub async fn update_user(&self, user: UserAttributes) -> Result<UserUpdate, Error> {
        let session = match &self.current_session {
            Some(s) => s,
            None => return Err(Error::NotAuthenticated),
        };

        let result = self.api.update_user(user, &session.access_token).await;

        match result {
            Ok(user) => return Ok(user),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::UserNotFound);
                }
                return Err(Error::InternalError);
            }
        }
    }

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

        return Ok(session);
    }
}
