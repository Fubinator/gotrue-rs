[![Rust](https://github.com/Fubinator/gotrue-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Fubinator/gotrue-rs/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/go_true.svg)](https://crates.io/crates/go_true)
[![License: MIT](https://img.shields.io/crates/l/go_true.svg)](#license)

This is a [GoTrue](https://github.com/supabase/gotrue) client implementation in rust. The library is currently under development. Most of the features are already built in, but there are still some changes to be made and everything still needs to be documented. 

## Usage
Add the following line to your `Cargo.toml`:

```toml
go_true = "0.1.0"
```

## Examples

WIP

## Testing

The first thing to do is to start the supabase server in docker:

```sh
cd infra
docker compose up
```

Once the server has been started, the tests can be run:

```sh
cargo test
```

## Contributing

Contributions, issues and feature requests are welcome. Feel free to check out the [issues page](https://github.com/Fubinator/gotrue-rs/issues) if you want to contribute.
