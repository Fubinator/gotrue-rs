use crate::{
    go_true_api::GoTrueApi, session::Session, user_attributes::UserAttributes,
    user_update::UserUpdate,
};

pub struct GoTrueClient {
    current_session: Option<Session>,
    auto_refresh_token: bool,
    api: GoTrueApi,
}

impl GoTrueClient {
    pub fn new(url: String) -> GoTrueClient {
        GoTrueClient {
            auto_refresh_token: true,
            current_session: None,
            api: GoTrueApi::new(url),
        }
    }

    pub async fn sign_up(&mut self, email: &String, password: &String) -> Session {
        let result = self.api.sign_up(&email, &password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return session;
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    pub async fn sign_in(&mut self, email: &String, password: &String) -> Session {
        let result = self.api.sign_in(&email, &password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return session;
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    pub async fn send_otp(&self, email: &str, should_create_user: Option<bool>) -> bool {
        let result = self.api.send_otp(&email, should_create_user).await;

        match result {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn sign_out(&self) -> bool {
        let result = match &self.current_session {
            Some(session) => self.api.sign_out(&session.access_token).await,
            None => return true,
        };

        match result {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn reset_password_for_email(&self, email: &str) -> bool {
        let result = self.api.reset_password_for_email(&email).await;

        match result {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn update_user(&self, user: UserAttributes) -> Result<UserUpdate, reqwest::Error> {
        let session = match &self.current_session {
            Some(s) => s,
            None => panic!("Not logged in"),
        };

        let result = self.api.update_user(user, &session.access_token).await?;

        return Ok(result);
    }
}
