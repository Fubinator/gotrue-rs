use serde::{Deserialize, Serialize};
use serde_json::Value;

///
/// Represents user attributes used in updating a user.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    /// The email of a user.
    pub email: String,
    /// The password of a user.
    pub password: String,
    /// Additional user data.
    pub data: Value,
}
