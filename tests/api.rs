use go_true::{go_true_api::GoTrueApi, user_attributes::UserAttributes};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use std::error::Error;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

fn get_api_client() -> GoTrueApi {
    let api: GoTrueApi = GoTrueApi::new(String::from("http://localhost:9998"));

    return api;
}

fn get_service_api_client() -> GoTrueApi {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"37c304f8-51aa-419a-a1af-06154e63707a").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "1234567890");
    claims.insert("role", "supabase_admin");

    let token_str = claims.sign_with_key(&key).unwrap();
    let api: GoTrueApi = GoTrueApi::new(String::from("http://localhost:9998"))
        .insert_header("Authorization", format!("Bearer {token_str}"));

    return api;
}

fn get_random_email() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(|c| c.to_ascii_lowercase())
        .map(char::from)
        .collect();

    return format!("{random_string}@example.com");
}

#[tokio::test]
async fn it_signs_up_with_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let res = api.sign_up(&email, &password).await?;

    assert_eq!(res.user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_signs_in_with_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password).await?;
    let res = api.sign_in(&email, &password).await?;

    assert_eq!(res.user.email, email);
    Ok(())
}

#[tokio::test]
async fn it_send_magic_link_with_valid_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password).await?;
    let res = api.send_otp(&email, None).await?;

    assert_eq!(res, true);

    Ok(())
}

#[tokio::test]
async fn it_does_not_send_magic_link_with_invalid_email() -> Result<(), Box<dyn Error>> {
    let email = String::from("i-do-not-exist");
    let api = get_api_client();
    let response = api.send_otp(&email, None).await;

    match response {
        Ok(_) => panic!("Should not work"),
        Err(_) => assert!(true),
    }

    Ok(())
}

#[tokio::test]
async fn it_should_log_out() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password).await?;
    let res = api.sign_in(&email, &password).await?;

    assert_eq!(res.user.email, email);

    let success = api.sign_out(&res.access_token).await?;

    assert_eq!(success, true);

    Ok(())
}

#[tokio::test]
async fn it_should_return_error_if_token_is_invalid() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password).await?;
    let res = api.sign_in(&email, &password).await?;

    assert_eq!(res.user.email, email);

    let success = api.sign_out(&"invalid-token".to_string()).await;

    match success {
        Ok(_) => panic!("Should not work"),
        Err(_) => assert!(true),
    }
    Ok(())
}

#[tokio::test]
async fn it_should_send_password_recovery_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password).await?;

    let success = api.reset_password_for_email(&email).await?;
    assert_eq!(success, true);

    Ok(())
}

#[tokio::test]
async fn it_should_return_error_in_password_recovery_if_email_was_not_found(
) -> Result<(), Box<dyn Error>> {
    let email = get_random_email();

    let api = get_api_client();
    let success = api.reset_password_for_email(&email).await;
    match success {
        Ok(_) => panic!("Should not work"),
        Err(_) => assert!(true),
    }

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
    api.sign_up(&email, &password).await?;
    let session = api.sign_in(&email, &password).await?;

    let new_session = api.refresh_access_token(&session.refresh_token).await?;

    assert_eq!(new_session.user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_should_return_user() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password).await?;
    let session = api.sign_in(&email, &password).await?;

    let user = api.get_user(&session.access_token).await?;

    assert_eq!(user.email, email);

    Ok(())
}

#[tokio::test]
async fn it_should_update_user() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password).await?;
    let session = api.sign_in(&email, &password).await?;

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
    let password = String::from("Abcd1234!");
    let client_api = get_api_client();
    client_api.sign_up(&email, &password).await?;

    let api = get_service_api_client();
    let users = api.list_users(None).await?;

    assert_eq!(users.users.is_empty(), false);

    Ok(())
}

#[tokio::test]
async fn it_should_get_user_by_id() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");
    let client_api = get_api_client();
    let session = client_api.sign_up(&email, &password).await?;

    let api = get_service_api_client();
    let user = api.get_user_by_id(&session.user.id).await?;

    assert_eq!(user.email, email);

    Ok(())
}
