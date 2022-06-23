use crate::go_true_api::GoTrueApi;

struct User {
    id: String,
    aud: String,
    confirmation_sent_at: String,
    recovery_sent_at: String,
    email_change_sent_at: String,
    new_email: String,
    invited_at: String,
    action_link: String,
    email: String,
    phone: String,
    created_at: String,
    confirmed_at: String,
    email_confirmed_at: String,
    phone_confirmed_at: String,
    last_sign_in_at: String,
    role: String,
    updated_at: String,
}

struct Session {
    provider_token: String,
    access_token: String,
    // The number of seconds until the token expires (since it was issued). Returned when a login is confirmed.
    expires_in: i32,
    // A timestamp of when the token will expire. Returned when a login is confirmed.
    expires_at: i32,
    refresh_token: String,
    token_type: String,
    user: User,
}

pub struct GoTrueClient {
    current_session: Session,
    auto_refresh_token: bool,
    api: GoTrueApi,
}

impl GoTrueClient {
    pub fn new() -> GoTrueClient {
        let user = User {
            id: String::from(""),
            aud: String::from(""),
            confirmation_sent_at: String::from(""),
            recovery_sent_at: String::from(""),
            email_change_sent_at: String::from(""),
            new_email: String::from(""),
            invited_at: String::from(""),
            action_link: String::from(""),
            email: String::from(""),
            phone: String::from(""),
            created_at: String::from(""),
            confirmed_at: String::from(""),
            email_confirmed_at: String::from(""),
            phone_confirmed_at: String::from(""),
            last_sign_in_at: String::from(""),
            role: String::from(""),
            updated_at: String::from(""),
        };

        GoTrueClient {
            auto_refresh_token: true,
            current_session: Session {
                provider_token: String::from(""),
                access_token: String::from(""),
                expires_in: 0,
                expires_at: 0,
                refresh_token: String::from(""),
                token_type: String::from(""),
                user,
            },
            api: GoTrueApi::new(String::from("Some kind of url")),
        }
    }

    // pub fn sign_up(email: str, password: str, phone: str) {}
}
