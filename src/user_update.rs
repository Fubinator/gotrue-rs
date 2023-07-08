use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub id: String,
    pub email: String,
    pub new_email: String,
    pub email_change_sent_at: String,
    pub created_at: String,
    pub updated_at: String,
}
