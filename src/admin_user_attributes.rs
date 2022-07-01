use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdminUserAttributes {
    pub email: String,
    pub password: Option<String>,
    pub data: Option<Value>,
    pub email_confirmed: Option<bool>,
    pub phone_confirmed: Option<bool>,
}
