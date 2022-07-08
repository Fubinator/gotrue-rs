use go_true::{Client, EmailOrPhone, UserAttributes};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use std::error::Error;

fn get_client() -> Client {
    return Client::new("http://localhost:9998".to_string());
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

    let mut client = get_client();
    let res = client.sign_up(&email, &password).await?;

    assert_eq!(email, res.user.email);

    Ok(())
}

#[tokio::test]
async fn it_should_throw_email_already_taken_error() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;

    let result = client.sign_up(&email, &password).await;

    match result {
        Ok(_) => panic!("Should throw error"),
        Err(e) => assert!(matches!(e, go_true::error::Error::AlreadySignedUp)),
    }

    Ok(())
}

#[tokio::test]
async fn it_signs_in_with_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;
    let res = client.sign_in(&email, &password).await?;

    assert_eq!(res.user.email, email);
    Ok(())
}

#[tokio::test]
async fn it_should_return_error_when_credentials_are_wrong_on_signin() -> Result<(), Box<dyn Error>>
{
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;

    let wrong_email = get_random_email();
    let result = client.sign_in(&wrong_email, &password).await;

    match result {
        Ok(_) => panic!("Should throw error"),
        Err(e) => assert!(matches!(e, go_true::error::Error::WrongCredentials)),
    }

    Ok(())
}

#[tokio::test]
async fn it_should_return_error_if_no_session_when_refreshing() -> Result<(), Box<dyn Error>> {
    let mut client = get_client();
    let result = client.refresh_session().await;

    match result {
        Ok(_) => panic!("Should throw error"),
        Err(e) => assert!(matches!(e, go_true::error::Error::NotAuthenticated)),
    }

    Ok(())
}

#[tokio::test]
async fn it_should_refresh_session() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;
    let old_session = client.sign_in(&email, &password).await?;

    let session = client.refresh_session().await?;

    assert_eq!(session.user.email, email);
    assert_ne!(old_session.refresh_token, session.refresh_token);

    Ok(())
}

#[tokio::test]
async fn it_send_magic_link_with_valid_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;
    let res = client.send_otp(EmailOrPhone::Email(email), None).await?;

    assert_eq!(res, true);
    Ok(())
}

#[tokio::test]
async fn it_does_not_send_magic_link_with_invalid_email() -> Result<(), Box<dyn Error>> {
    let email = String::from("i-do-not-exist");
    let client = get_client();
    let result = client.send_otp(EmailOrPhone::Email(email), None).await;

    match result {
        Ok(_) => panic!("Should throw error"),
        Err(e) => assert!(matches!(e, go_true::error::Error::UserNotFound)),
    }

    Ok(())
}

#[tokio::test]
async fn it_should_log_out() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;
    client.sign_in(&email, &password).await?;

    let success = client.sign_out().await?;

    assert_eq!(success, true);
    Ok(())
}

#[tokio::test]
async fn it_should_return_error_in_log_out_if_no_session() -> Result<(), Box<dyn Error>> {
    let client = get_client();
    let result = client.sign_out().await;

    match result {
        Ok(_) => panic!("Should throw error"),
        Err(e) => assert!(matches!(e, go_true::error::Error::NotAuthenticated)),
    }

    Ok(())
}

#[tokio::test]
async fn it_should_send_password_recovery_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;
    let res = client.reset_password_for_email(&email).await;

    assert_eq!(res, true);
    Ok(())
}

#[tokio::test]
async fn it_should_return_false_if_email_was_not_found_in_password_recovery(
) -> Result<(), Box<dyn Error>> {
    let email = get_random_email();

    let client = get_client();
    let res = client.reset_password_for_email(&email).await;

    assert_eq!(res, false);
    Ok(())
}

#[tokio::test]
async fn it_should_update_user() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await?;
    client.sign_in(&email, &password).await?;

    let new_email = get_random_email();
    let attributes = UserAttributes {
        email: new_email.clone(),
        password: "Abcd12345!".to_string(),
        data: json!({ "test": "test" }),
    };

    let update = client.update_user(attributes).await?;

    assert_eq!(update.new_email, new_email);

    Ok(())
}
