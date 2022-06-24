use reqwest::{
    blocking::Response,
    header::{HeaderMap, HeaderValue, IntoHeaderName},
    StatusCode,
};
use serde_json::json;
use urlencoding::encode;

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

    pub fn sign_up(
        self,
        email: &String,
        password: &String,
        redirect_to: Option<String>,
    ) -> Result<Session, reqwest::Error> {
        let query_string = match redirect_to {
            Some(query) => format!("?redirect_to={}", encode(&query)),
            _ => String::from(""),
        };

        let endpoint = format!("{}/signup{}", self.url, query_string);

        let body = json!({
            "email": &email,
            "password": &password,
        });

        let client = reqwest::blocking::Client::new();
        let response: Session = client
            .post(endpoint)
            .headers(self.headers)
            .json(&body)
            .send()
            .unwrap()
            .error_for_status()?
            .json()
            .unwrap();

        return Ok(response);
    }

    pub fn sign_in(
        self,
        email: &String,
        password: &String,
        redirect_to: Option<String>,
    ) -> Result<Session, reqwest::Error> {
        let query_string = match redirect_to {
            Some(query) => format!("?grant_type=password&redirect_to={}", encode(&query)),
            _ => String::from("?grant_type=password"),
        };

        let endpoint = format!("{}/token{}", self.url, query_string);

        let body = json!({
            "email": &email,
            "password": &password,
        });

        let client = reqwest::blocking::Client::new();
        let response: Session = client
            .post(endpoint)
            .headers(self.headers)
            .json(&body)
            .send()
            .unwrap()
            .error_for_status()?
            .json()
            .unwrap();

        return Ok(response);
    }
}
