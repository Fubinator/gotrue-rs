use go_true::{Api, EmailOrPhone, UserAttributes};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use std::error::Error;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AdminUserAttributes {
    pub email: String,
    pub password: Option<String>,
    pub data: Option<Value>,
    pub email_confirmed: Option<bool>,
    pub phone_confirmed: Option<bool>,
}

fn get_api_client() -> Api {
    Api::new("http://localhost:9998")
}

fn get_service_api_client() -> Api {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"37c304f8-51aa-419a-a1af-06154e63707a").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "1234567890");
    claims.insert("role", "supabase_admin");

    let token_str = claims.sign_with_key(&key).unwrap();
    let api: Api = Api::new("http://localhost:9998")
        .insert_header("Authorization", format!("Bearer {token_str}"));

    api
}

fn get_random_email() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(|c| c.to_ascii_lowercase())
        .map(char::from)
        .collect();

    format!("{random_string}@example.com")
}

#[tokio::test]
async fn it_signs_up_with_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let res = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;

    assert_eq!(res.user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_signs_in_with_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;
    let res = api
        .sign_in(EmailOrPhone::Email(email.clone()), &password)
        .await?;

    assert_eq!(res.user.email, email);
    Ok(())
}

#[tokio::test]
async fn it_send_magic_link_with_valid_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;
    let res = api
        .send_otp(EmailOrPhone::Email(email.clone()), None)
        .await?;

    assert!(res);

    Ok(())
}

#[tokio::test]
async fn it_does_not_send_magic_link_with_invalid_email() -> Result<(), Box<dyn Error>> {
    let email = String::from("i-do-not-exist");
    let api = get_api_client();
    let response = api.send_otp(EmailOrPhone::Email(email), None).await;

    match response {
        Ok(_) => panic!("Should not work"),
        Err(_) => Ok(()),
    }
}

#[tokio::test]
async fn it_should_log_out() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;
    let res = api
        .sign_in(EmailOrPhone::Email(email.clone()), &password)
        .await?;

    assert_eq!(res.user.email, email);

    let success = api.sign_out(&res.access_token).await?;

    assert!(success);

    Ok(())
}

#[tokio::test]
async fn it_should_return_error_if_token_is_invalid() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;
    let res = api
        .sign_in(EmailOrPhone::Email(email.clone()), &password)
        .await?;

    assert_eq!(res.user.email, email);

    let success = api.sign_out("invalid-token").await;

    match success {
        Ok(_) => panic!("Should not work"),
        Err(_) => Ok(()),
    }
}

#[tokio::test]
async fn it_should_send_password_recovery_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;

    let success = api.reset_password_for_email(&email).await?;
    assert!(success);

    Ok(())
}

#[test]
fn it_should_return_url_for_provider() {
    let api = get_api_client();
    let url = api.get_url_for_provider("Github");

    assert!(url.ends_with("/authorize?provider=Github"));
}

#[tokio::test]
async fn it_should_refresh_token() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;
    let session = api
        .sign_in(EmailOrPhone::Email(email.clone()), &password)
        .await?;

    let new_session = api.refresh_access_token(&session.refresh_token).await?;

    assert_eq!(new_session.user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_should_return_user() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;
    let session = api
        .sign_in(EmailOrPhone::Email(email.clone()), &password)
        .await?;

    let user = api.get_user(&session.access_token).await?;

    assert_eq!(user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_should_update_user() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let _throw_away_signup_result = api
        .sign_up(EmailOrPhone::Email(email.clone()), &password)
        .await?;
    let session = api.sign_in(EmailOrPhone::Email(email), &password).await?;

    let new_email = get_random_email();
    let attributes = UserAttributes {
        email: new_email.clone(),
        password: "Abcd12345!".to_string(),
        data: json!({ "test": "test" }),
    };

    let update = api.update_user(attributes, &session.access_token).await?;

    assert_eq!(update.new_email, new_email);

    Ok(())
}

#[tokio::test]
async fn it_should_invite_user_by_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let api = get_service_api_client();
    let user = api.invite_user_by_email(&email).await?;

    assert_eq!(user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_should_list_users() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = "Abcd1234!";
    let client_api = get_api_client();
    let _throw_away_signup_result = client_api
        .sign_up(EmailOrPhone::Email(email), password)
        .await?;

    let api = get_service_api_client();
    let users = api.list_users(None).await?;

    assert!(!users.users.is_empty());

    Ok(())
}

#[tokio::test]
async fn it_should_get_user_by_id() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = "Abcd1234!";
    let client_api = get_api_client();
    let session = client_api
        .sign_up(EmailOrPhone::Email(email.clone()), password)
        .await?;

    let api = get_service_api_client();
    let user = api.get_user_by_id(&session.user.id).await?;

    assert_eq!(user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_should_create_user() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let api = get_service_api_client();
    let user = AdminUserAttributes {
        email: email.clone(),
        password: Some("Abcd1234!".to_owned()),
        data: None,
        email_confirmed: None,
        phone_confirmed: None,
    };

    let response = api.create_user(user).await?;

    assert_eq!(response.email, email);

    Ok(())
}

#[tokio::test]
async fn it_should_update_user_by_id() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let api = get_service_api_client();
    let user = AdminUserAttributes {
        email: email.clone(),
        password: Some("Abcd1234!".to_owned()),
        data: Some(serde_json::Value::Null),
        email_confirmed: None,
        phone_confirmed: None,
    };

    let create_response = api.create_user(user).await?;
    assert_eq!(create_response.email, email);

    let new_email = get_random_email();

    let user = AdminUserAttributes {
        email: new_email.clone(),
        password: None,
        data: None,
        email_confirmed: None,
        phone_confirmed: None,
    };

    let update_response = api
        .update_user_by_id(&create_response.id, user.clone())
        .await?;

    assert_eq!(update_response.email, new_email);

    Ok(())
}

#[tokio::test]
async fn it_should_delete_user() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let api = get_service_api_client();
    let user = AdminUserAttributes {
        email: email.clone(),
        password: Some("Abcd1234!".to_owned()),
        data: Some(serde_json::Value::Null),
        email_confirmed: None,
        phone_confirmed: None,
    };

    let create_response = api.create_user(user).await?;
    assert_eq!(create_response.email, email);

    let old_user_list = api.list_users(None).await?;

    api.delete_user(&create_response.id).await?;
    assert!(old_user_list.users.iter().any(|user| user.email == email));

    let userlist = api.list_users(None).await?;

    assert!(!userlist.users.iter().any(|user| user.email == email));

    Ok(())
}
