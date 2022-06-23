use dotenv;
use go_true::go_true_api::GoTrueApi;

fn getApiCLient() -> GoTrueApi {
    dotenv::from_filename(".env").ok();
    let api: GoTrueApi = GoTrueApi::new(dotenv::var("SUPABASE_URL").unwrap())
        .insert_header("apikey", dotenv::var("SUPABASE_API_KEY").unwrap())
        .insert_header(
            "Authorization",
            format!("Bearer {}", dotenv::var("SUPABASE_API_KEY").unwrap()),
        );

    return api;
}

#[test]
fn it_signs_up_with_email() {
    let email = String::from("test@test.de");
    let password = String::from("Abcd1234!");
    let api = getApiCLient();

    let res = api.sign_up_with_email(&email, &password, None).unwrap();
    assert_eq!(1, 1);
}

#[test]
fn it_signs_in_with_email() {
    let email = String::from("test@test.de");
    let password = String::from("Abcd1234!");
    let api = getApiCLient();

    let res = api.sign_in_with_email(&email, &password, None).unwrap();
    assert_eq!(1, 1);
}
