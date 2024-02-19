use std::error::Error;

use go_true::Api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:9999";

    let mut client = Api::new(url);

    let g_url = client.get_url_for_provider("github");

    println!("Go here to sign in local:\n{}", g_url);

    let session = client.provider_sign_in().await.expect("Uhoh signin broke!");

    println!("Local Login Complete: \n{:?}", session);

    Ok(())
}
