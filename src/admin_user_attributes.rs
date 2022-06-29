use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminUserAttributes {
    pub email: String,
    pub password: String,
    pub data: Value,
    pub email_confirmed: Option<bool>,
    pub phone_confirmed: Option<bool>,
}
