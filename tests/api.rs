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
