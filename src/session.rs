use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Session {
    id: String,
    aud: String,
    role: String,
    email: String,
    created_at: String,
    updated_at: String,
}
