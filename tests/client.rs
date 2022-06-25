use go_true::go_true_client::GoTrueClient;
use rand::{distributions::Alphanumeric, Rng};

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

#[test]
fn it_signs_up_with_email() {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    let res = client.sign_up(&email, &password, None);

    assert_eq!(res.user.email, email);
}

#[test]
fn it_signs_in_with_email() {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password, None);
    let res = client.sign_in(&email, &password, None);

    assert_eq!(res.user.email, email);
}

#[test]
fn it_send_magic_link_with_valid_email() {
    let email = get_random_email();
    let password = String::from("Abcd1234!");

    let mut client = get_client();
    client.sign_up(&email, &password, None);
    let res = client.send_otp(&email, None, None);

    assert_eq!(res, true);
}

#[test]
fn it_does_not_send_magic_link_with_invalid_email() {
    let email = String::from("i-do-not-exist");
    let client = get_client();
    let res = client.send_otp(&email, None, None);

    assert_eq!(res, false);
}
