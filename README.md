# gotrue-rs

[![Rust](https://github.com/Fubinator/gotrue-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Fubinator/gotrue-rs/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/go_true.svg)](https://crates.io/crates/go_true)
[![License: MIT](https://img.shields.io/crates/l/go_true.svg)](#license)

This is a [GoTrue](https://github.com/supabase/gotrue) client implementation in rust. The library is currently under development. Most of the features are already built in, but there are still some changes to be made and everything still needs to be documented.

## Usage

Add the following line to your `Cargo.toml`:

```toml
go_true = "0.1.1"
```

## Examples

To create an account, create a new client and execute the `sign_up` function with email and password:

```rust
use go_true::{Client, EmailOrPhone};

#[tokio::main]
async fn main() {
    let mut client = Client::new("http://localhost:9998");

    let email = "email@example.com".to_string();
    let password = "Abcd1234!";

    let session = client.sign_up(EmailOrPhone::Email(email), password).await;

    println!("{:?}", session);
}
```

For more information, check out the [API docs](https://docs.rs/go_true/0.1.0/go_true/)!

## Testing

The first thing to do is to start the supabase server in docker:

```sh
cd infra
docker compose up
```

Once the server has been started, the tests can be run:

```sh
cargo test --tests
```

## Contributing

Contributions, issues and feature requests are welcome. Feel free to check out the [issues page](https://github.com/Fubinator/gotrue-rs/issues) if you want to contribute.
