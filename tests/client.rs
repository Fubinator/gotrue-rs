use go_true::{GoTrueClient, UserAttributes};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use std::error::Error;

fn get_client() -> GoTrueClient {
    return GoTrueClient::new("http://localhost:9998".to_string());
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
    let res = client.sign_up(&email, &password).await;

    assert_eq!(res.user.email, email);
    Ok(())
}

#[tokio::test]
async fn it_signs_in_with_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await;
    let res = client.sign_in(&email, &password).await;

    assert_eq!(res.user.email, email);
    Ok(())
}

#[tokio::test]
async fn it_send_magic_link_with_valid_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await;
    let res = client.send_otp(&email, None).await;

    assert_eq!(res, true);
    Ok(())
}

#[tokio::test]
async fn it_does_not_send_magic_link_with_invalid_email() -> Result<(), Box<dyn Error>> {
    let email = String::from("i-do-not-exist");
    let client = get_client();
    let res = client.send_otp(&email, None).await;

    assert_eq!(res, false);
    Ok(())
}

#[tokio::test]
async fn it_should_log_out() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await;
    client.sign_in(&email, &password).await;

    let success = client.sign_out().await;

    assert_eq!(success, true);
    Ok(())
}

#[tokio::test]
async fn it_should_send_password_recovery_email() -> Result<(), Box<dyn Error>> {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password).await;
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
    client.sign_up(&email, &password).await;
    client.sign_in(&email, &password).await;

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
