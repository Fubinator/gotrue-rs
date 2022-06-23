use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use serde_json::{json, Value};

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

    pub fn sign_up_with_email(
        self,
        email: &String,
        password: &String,
        redirect_to: Option<String>,
        captcha_token: Option<String>,
    ) -> Result<String, reqwest::Error> {
        // let query_string: String = redirect_to.unwrap_or(String::from(""));
        let body = json!({
            "email": &email,
            "password": &password
        });

        print!("{}", body);

        let endpoint = format!("{}/signup", self.url);

        let client = reqwest::blocking::Client::new();
        let res: reqwest::blocking::Response = client
            .post(endpoint)
            .headers(self.headers)
            .json(&body)
            .send()
            .unwrap();

        println!("{}", res.text().unwrap());

        return Ok(String::from("ASD"));
    }
}
