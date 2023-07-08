use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserList {
    pub users: Vec<User>,
}
