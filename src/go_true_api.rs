use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use serde_json::json;

use crate::session::Session;

pub struct GoTrueApi {
    url: String,
    headers: HeaderMap,
}

impl GoTrueApi {
    pub fn new(url: String) -> GoTrueApi {
        GoTrueApi {
            url,
            headers: HeaderMap::new(),
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

        let client = reqwest::Client::new();
        let response: Session = client
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

        let client = reqwest::Client::new();
        let response: Session = client
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

        let client = reqwest::Client::new();
        client
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

        let client = reqwest::Client::new();
        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {access_token}");
        headers.insert(
            "Authorization",
            HeaderValue::from_str(bearer.as_ref()).expect("Invalid header value."),
        );

        client
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

        let client = reqwest::Client::new();
        client
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
}
