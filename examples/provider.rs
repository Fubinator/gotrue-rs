use std::error::Error;

use go_true::Api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:9999";

    let mut client = Api::new(url);

    let g_url = client.get_url_for_provider("google");

    println!("Go here to sign in:\n{}", g_url);

    let session = client.provider_sign_in().await.expect("Uhoh signin broke!");

    println!("{:?}", session);

    Ok(())
}
