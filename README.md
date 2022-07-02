[![Rust](https://github.com/Fubinator/gotrue-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Fubinator/gotrue-rs/actions/workflows/ci.yml)

## Getting Started

This is a [GoTrue](https://github.com/supabase/gotrue) client implementation in rust. Currently WIP (see the Status section for the current progress).

## Status

- [ ] Functionality
  - [x] Sign Up with Email/Phone
  - [x] Sign In with Email/Phone
  - [x] Send Magic Link Email
  - [x] Sign out
  - [x] Verify OTP
  - [x] Reset Password for Email
  - [x] Get Url for Provider
  - [x] Get User
  - [x] Update User
  - [x] Refresh Access Token
  - [x] Invite User by Email
  - [x] List Users (includes filtering, sorting, pagination)
  - [x] Get User by Id
  - [x] Create User
  - [x] Update User by Id
  - [x] Delete User
  - [ ] Refresh Session
  - [ ] Auth State Change Handler
  - [ ] Provider Sign In (Provides URL)
- [ ] Documentation
- [x] Unit Tests
- [ ] Cargo Release

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

We are more than happy to have contributions! Please submit a PR.
