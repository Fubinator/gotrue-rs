use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use serde_json::json;

use crate::{
    session::Session, user::User, user_attributes::UserAttributes, user_list::UserList,
    user_update::UserUpdate,
};

pub struct GoTrueApi {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}

impl GoTrueApi {
    pub fn new(url: String) -> GoTrueApi {
        GoTrueApi {
            url,
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }

    pub fn insert_header(
        mut self,
        header_name: impl IntoHeaderName,
        header_value: impl AsRef<str>,
    ) -> Self {
        self.headers.insert(
            header_name,
            HeaderValue::from_str(header_value.as_ref()).expect("Invalid header value."),
        );
        self
    }

    pub async fn sign_up(
        &self,
        email: &String,
        password: &String,
    ) -> Result<Session, reqwest::Error> {
        let endpoint = format!("{}/signup", self.url);

        let body = json!({
            "email": &email,
            "password": &password,
        });

        let response: Session = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<Session>()
            .await?;

        return Ok(response);
    }

    pub async fn sign_in(
        &self,
        email: &String,
        password: &String,
    ) -> Result<Session, reqwest::Error> {
        let query_string = String::from("?grant_type=password");

        let endpoint = format!("{}/token{}", self.url, query_string);

        let body = json!({
            "email": &email,
            "password": &password,
        });

        let response: Session = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<Session>()
            .await?;

        return Ok(response);
    }

    pub async fn send_otp(
        &self,
        email: &str,
        should_create_user: Option<bool>,
    ) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/otp", self.url);

        let body = json!({
            "email": &email,
            "should_create_user": Some(should_create_user)
        });

        self.client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        return Ok(true);
    }

    pub async fn sign_out(&self, access_token: &String) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/logout", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {access_token}");
        headers.insert(
            "Authorization",
            HeaderValue::from_str(bearer.as_ref()).expect("Invalid header value."),
        );

        self.client
            .post(endpoint)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        return Ok(true);
    }

    pub async fn reset_password_for_email(&self, email: &str) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/recover", self.url);

        let body = json!({
            "email": &email,
        });

        self.client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        return Ok(true);
    }

    pub fn get_url_for_provider(&self, provider: &str) -> String {
        return format!("{}/authorize?provider={}", self.url, provider);
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<Session, reqwest::Error> {
        let endpoint = format!("{}/token?grant_type=refresh_token", self.url);
        let body = json!({ "refresh_token": refresh_token });

        let session: Session = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(session);
    }

    pub async fn get_user(&self, jwt: &str) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/user", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {jwt}");
        headers.insert(
            "Authorization",
            HeaderValue::from_str(bearer.as_ref()).expect("Invalid header value."),
        );

        let user: User = self
            .client
            .get(endpoint)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(user);
    }

    pub async fn update_user(
        &self,
        user: UserAttributes,
        jwt: &str,
    ) -> Result<UserUpdate, reqwest::Error> {
        let endpoint = format!("{}/user", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {jwt}");
        headers.insert(
            "Authorization",
            HeaderValue::from_str(bearer.as_ref()).expect("Invalid header value."),
        );

        let body = json!({"email": user.email, "password": user.password, "data": user.data});

        let user: UserUpdate = self
            .client
            .put(endpoint)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<UserUpdate>()
            .await?;

        return Ok(user);
    }

    pub async fn invite_user_by_email(&self, email: &str) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/invite", self.url);

        let body = json!({
            "email": &email,
        });

        let user: User = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<User>()
            .await?;

        return Ok(user);
    }

    pub async fn list_users(
        &self,
        query_string: Option<String>,
    ) -> Result<UserList, reqwest::Error> {
        let endpoint = match query_string {
            Some(query) => format!("{}/admin/users{}", self.url, query),
            None => format!("{}/admin/users", self.url),
        };

        let users: UserList = self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(users);
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, user_id);

        let user: User = self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(user);
    }

    pub async fn create_user<T: serde::Serialize>(&self, user: T) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/admin/users", self.url);

        let json = serde_json::to_value(&user).unwrap();

        let client = reqwest::Client::new();
        let user: User = client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&json)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(user);
    }

    pub async fn update_user_by_id<T: serde::Serialize>(
        &self,
        id: &str,
        user: T,
    ) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, id);

        let json = serde_json::to_value(&user).unwrap();

        let client = reqwest::Client::new();
        let user: User = client
            .put(endpoint)
            .headers(self.headers.clone())
            .json(&json)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(user);
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, user_id);

        self.client
            .delete(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?;

        return Ok(true);
    }
}
