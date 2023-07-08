use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    pub email: String,
    pub password: String,
    pub data: Value,
}
