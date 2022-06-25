use crate::{go_true_api::GoTrueApi, session::Session};

#[derive(Clone)]
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

    pub fn sign_up(
        self,
        email: &String,
        password: &String,
        redirect_to: Option<String>,
    ) -> Result<Session, reqwest::Error> {
        return self.api.sign_up(&email, &password, redirect_to);
    }

    pub fn sign_in(
        self,
        email: &String,
        password: &String,
        redirect_to: Option<String>,
    ) -> Result<Session, reqwest::Error> {
        return self.api.sign_in(&email, &password, redirect_to);
    }
}
