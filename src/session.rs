use serde::Deserialize;

use crate::user::User;

#[derive(Debug, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub user: User,
}
