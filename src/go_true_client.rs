use crate::{go_true_api::GoTrueApi, session::Session};

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
        &mut self,
        email: &String,
        password: &String,
        redirect_to: Option<String>,
    ) -> Session {
        let result = self.api.sign_up(&email, &password, redirect_to);

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return session;
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn sign_in(
        &mut self,
        email: &String,
        password: &String,
        redirect_to: Option<String>,
    ) -> Session {
        let result = self.api.sign_in(&email, &password, redirect_to);

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return session;
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
