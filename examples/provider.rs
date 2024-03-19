use std::{env, error::Error};

use go_true::Api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://{}.supabase.co/auth/v1",
        env::var("SUPABASE_ID").unwrap()
    );

    let mut client = Api::new(url).insert_header("apikey", env::var("ANON_KEY").unwrap());

    let g_url = client.get_url_for_provider("github");

    println!("Go here to sign in live:\n{}", g_url);

    let session = client.provider_sign_in().await.expect("Uhoh signin broke!");

    println!("Login Complete: \n{:?}", session);

    Ok(())
}
