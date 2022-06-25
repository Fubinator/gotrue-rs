use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub aud: String,
    pub role: String,
    pub email_confirmed_at: String,
    pub phone: String,
    pub last_sign_in_at: String,
    pub created_at: String,
    pub updated_at: String,
}
