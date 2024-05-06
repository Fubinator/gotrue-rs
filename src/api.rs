use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::TcpListener,
};

use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use serde_json::json;

use crate::{
    session::Session, user::User, user_attributes::UserAttributes, user_list::UserList,
    user_update::UserUpdate,
};

pub struct Api {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}

pub enum EmailOrPhone {
    Email(String),
    Phone(String),
}

impl Api {
    /// Creates a GoTrue API client.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::Api;
    ///
    /// let client = Api::new("http://your.gotrue.endpoint");
    /// ```
    pub fn new(url: impl Into<String>) -> Api {
        Api {
            url: url.into(),
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }

    /// Add arbitrary headers to the request. For instance when you may want to connect
    /// through an API gateway that needs an API key header.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::Api;
    ///
    /// let client = Api::new("https://your.gotrue.endpoint")
    ///     .insert_header("apikey", "super.secret.key");
    /// ```
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

    /// Signs up for a new account
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     let result = client.sign_up(EmailOrPhone::Email(email), password).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_up(
        &self,
        email_or_phone: EmailOrPhone,
        password: impl AsRef<str>,
    ) -> Result<Session, reqwest::Error> {
        let endpoint = format!("{}/signup", self.url);

        let body = match email_or_phone {
            EmailOrPhone::Email(email) => json!({
                "email": email,
                "password": password.as_ref(),
            }),
            EmailOrPhone::Phone(phone) => json!({
                "phone": phone,
                "password": password.as_ref()
            }),
        };

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

    /// Signs into an existing account
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     let result = client.sign_in(EmailOrPhone::Email(email), password).await;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_in(
        &self,
        email_or_phone: EmailOrPhone,
        password: impl AsRef<str>,
    ) -> Result<Session, reqwest::Error> {
        let query_string = String::from("?grant_type=password");

        let endpoint = format!("{}/token{}", self.url, query_string);

        let body = match email_or_phone {
            EmailOrPhone::Email(email) => json!({
                "email": email,
                "password": password.as_ref(),
            }),
            EmailOrPhone::Phone(phone) => json!({
                "phone": phone,
                "password": password.as_ref()
            }),
        };

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

    /// Signs in with a provider
    ///
    /// Appropriate URI should be presented to the user before this function is called.
    ///
    /// # Example
    ///
    /// TODO
    pub async fn provider_sign_in(&mut self) -> Result<Session, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:6969").expect("Couldn't bind port 6969.");

        let mut params = HashMap::new();

        loop {
            let (mut stream, _) = listener.accept().expect("Listener IO error");

            // This javascript is mental, I have to make fetch happen because GoTrue puts the
            // access token in the URI hash? Like is that intentional, surely should be on search
            // params. This fix does require JS in browser but most oAuth sign in pages probably do too, so
            // should be a non-issue.
            let message = String::from(
                "<script>fetch(`http://localhost:6969/token?${window.location.hash.replace('#','')})`)</script><h1>GoTrue-Rs</h1><h2>Signin sent to program.</h2><h3>You may close this tab.</h3>",
            );

            // TODO optional redirect to user provided URI

            let res = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );

            loop {
                match stream.write(res.as_bytes()) {
                    Ok(_n) => break,
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
                    Err(e) => println!("Couldn't respond. {}", e),
                }
            }

            let mut buf = [0; 4096];

            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(_n) => break,
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }

            let raw = String::from_utf8(buf.to_vec()).unwrap();

            let request_line = raw.lines().collect::<Vec<_>>()[0];

            if !request_line.starts_with("GET /token?") {
                // If this request isn't the one we sent with JS fetch, ignore it and wait for the
                // right one.
                continue;
            }

            let split_req = request_line
                .strip_prefix("GET /token?")
                .unwrap()
                .split('&')
                .collect::<Vec<&str>>();

            for param in split_req {
                let split_param = param.split('=').collect::<Vec<&str>>();
                params.insert(split_param[0].to_owned(), split_param[1].to_owned());
            }

            if params.get("access_token").is_some() {
                break;
            }
        }

        let access_token = params.get("access_token").unwrap().clone();
        let refresh_token = params.get("refresh_token").unwrap().clone();

        let sesh = Session {
            user: self.get_user(access_token.clone()).await?,
            access_token,
            refresh_token,
            token_type: "JWT".into(),
            expires_in: 3600, // TODO get correct time from params
        };

        Ok(sesh)
    }

    /// Sends an OTP Code and creates user if it does not exist
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///
    ///     let result = client.send_otp(EmailOrPhone::Email(email), None).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_otp(
        &self,
        email_or_phone: EmailOrPhone,
        should_create_user: Option<bool>,
    ) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/otp", self.url);

        let body = match email_or_phone {
            EmailOrPhone::Email(email) => json!({
                "email": email,
                "should_create_user": Some(should_create_user)
            }),
            EmailOrPhone::Phone(phone) => json!({
                "phone": phone,
                "should_create_user": Some(should_create_user)
            }),
        };

        self.client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        return Ok(true);
    }

    pub async fn verify_otp<T: serde::Serialize>(&self, params: T) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/verify", self.url);

        let body = serde_json::to_value(&params).unwrap();

        self.client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        return Ok(true);
    }

    /// Signs the current user out
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     let session = client.sign_in(EmailOrPhone::Email(email), password).await?;
    ///     client.sign_out(&session.access_token);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_out(&self, access_token: impl AsRef<str>) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/logout", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {}", access_token.as_ref());
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

    /// Sends password recovery email
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// let mut client = Api::new("http://localhost:9998");
    /// let email = "random@mail.com";
    ///
    /// client.reset_password_for_email(email);
    /// ```
    pub async fn reset_password_for_email(
        &self,
        email: impl AsRef<str>,
    ) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/recover", self.url);

        let body = json!({
            "email": email.as_ref(),
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

    /// Refreshes the current session by refresh token
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     let session = client.sign_in(EmailOrPhone::Email(email), password).await?;
    ///     client.refresh_access_token(&session.refresh_token);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn refresh_access_token(
        &self,
        refresh_token: impl AsRef<str>,
    ) -> Result<Session, reqwest::Error> {
        let endpoint = format!("{}/token?grant_type=refresh_token", self.url);
        let body = json!({ "refresh_token": refresh_token.as_ref() });

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

    /// Gets a user by access token
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     let session = client.sign_in(EmailOrPhone::Email(email), password).await?;
    ///     let user = client.get_user(&session.access_token);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_user(&self, jwt: impl AsRef<str>) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/user", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {}", jwt.as_ref());
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

    /// Updates a user
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone, UserAttributes};
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     client.sign_up(EmailOrPhone::Email(email.clone()), password)
    ///         .await?;
    ///     let session = client.sign_in(EmailOrPhone::Email(email), password).await?;
    ///
    ///     let new_email = "otheremail@example.com";
    ///     let attributes = UserAttributes {
    ///         email: new_email.to_string(),
    ///         password: "Abcd12345!".to_string(),
    ///         data: json!({ "test": "test" }),
    ///     };
    ///
    ///     let updated_user = client.update_user(attributes, &session.access_token).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_user(
        &self,
        user: UserAttributes,
        jwt: impl AsRef<str>,
    ) -> Result<UserUpdate, reqwest::Error> {
        let endpoint = format!("{}/user", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {}", jwt.as_ref());
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

    /// Invites a user via email
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com";
    ///
    ///     let user = client.invite_user_by_email(email).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn invite_user_by_email(
        &self,
        email: impl AsRef<str>,
    ) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/invite", self.url);

        let body = json!({
            "email": email.as_ref(),
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

    /// Lists all users based on a query string
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     client
    ///         .sign_up(EmailOrPhone::Email(email), password)
    ///         .await?;
    ///
    ///     let users = client.list_users(None).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Gets a user by id
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     let session = client
    ///         .sign_up(EmailOrPhone::Email(email), password)
    ///         .await?;
    ///
    ///     let user = client.get_user_by_id(&session.user.id).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_user_by_id(&self, user_id: impl AsRef<str>) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, user_id.as_ref());

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

    /// Creates a user
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let user = AdminUserAttributes {
    ///         email: "createemail@example.com".to_string(),
    ///         password: Some(String::from("Abcd1234!")),
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };

    ///     client.create_user(user).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Updates a user by id
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let user = AdminUserAttributes {
    ///         email: "oldemail@example.com",
    ///         password: Some(String::from("Abcd1234!")),
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };
    ///
    ///     let create_response = client.create_user(user).await?;
    ///     let user = AdminUserAttributes {
    ///         email: "newemail@example.com".to_string(),
    ///         password: None,
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };
    ///
    ///     let update_response = client
    ///         .update_user_by_id(&create_response.id, user.clone())
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_user_by_id<T: serde::Serialize>(
        &self,
        id: impl AsRef<str>,
        user: T,
    ) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, id.as_ref());

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

    /// Deletes a user by id
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Api::new("http://localhost:9998");
    ///
    ///     let user = AdminUserAttributes {
    ///         email: "delete@example.com".to_string(),
    ///         password: Some(String::from("Abcd1234!")),
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };

    ///     let user = client.create_user(user).await?;
    ///     client.delete_user(&user.id).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_user(&self, user_id: impl AsRef<str>) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, user_id.as_ref());

        self.client
            .delete(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?;

        return Ok(true);
    }
}
