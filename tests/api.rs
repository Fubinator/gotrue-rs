use dotenv;
use go_true::go_true_api::GoTrueApi;

#[test]
fn it_signs_up_with_email() {
    dotenv::from_filename(".env").ok();
    let api: GoTrueApi = GoTrueApi::new(dotenv::var("SUPABASE_URL").unwrap())
        .insert_header("apikey", dotenv::var("SUPABASE_API_KEY").unwrap())
        .insert_header(
            "Authorization",
            format!("Bearer {}", dotenv::var("SUPABASE_API_KEY").unwrap()),
        );

    let email = String::from("test@test.de");
    let password = String::from("Abcd1234!");

    let res = api.sign_up_with_email(&email, &password, None).unwrap();
    assert_eq!(1, 1);
}
