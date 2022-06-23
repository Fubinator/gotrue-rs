mod go_true_api;
mod go_true_client;
use crate::go_true_client::GoTrueClient;

fn main() {
    let client: GoTrueClient = GoTrueClient::new();
}
