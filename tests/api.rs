use go_true::go_true_api::GoTrueApi;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;

fn get_api_client() -> GoTrueApi {
    let api: GoTrueApi = GoTrueApi::new("http://localhost:9998".to_string());

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
