use go_true::go_true_api::GoTrueApi;
use rand::{distributions::Alphanumeric, Rng};

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

#[test]
fn it_signs_up_with_email() {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    let res = api.sign_up(&email, &password, None).unwrap();

    assert_eq!(res.user.email, email);
}

#[test]
fn it_signs_in_with_email() {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password, None).unwrap();
    let res = api.sign_in(&email, &password, None).unwrap();

    assert_eq!(res.user.email, email);
}

#[test]
fn it_send_magic_link_with_valid_email() {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password, None).unwrap();
    let res = api.send_otp(&email, None, None).unwrap();

    assert_eq!(res, true);
}

#[test]
fn it_does_not_send_magic_link_with_invalid_email() {
    let email = String::from("i-do-not-exist");
    let api = get_api_client();
    let response = api.send_otp(&email, None, None);

    match response {
        Ok(_) => panic!("Should not work"),
        Err(_) => assert!(true),
    }
}

#[test]
fn it_should_log_out() {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let api = get_api_client();
    api.sign_up(&email, &password, None).unwrap();
    let res = api.sign_in(&email, &password, None).unwrap();

    assert_eq!(res.user.email, email);

    println!("{}", res.access_token);

    let success = api.sign_out(&res.access_token).unwrap();

    assert_eq!(success, true);
}
