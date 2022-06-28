use serde_json::Value;

#[derive(Debug)]
pub struct UserAttributes {
    pub email: String,
    pub password: String,
    pub data: Value,
}
