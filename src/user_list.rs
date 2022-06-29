use crate::user::User;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserList {
    pub users: Vec<User>,
}
