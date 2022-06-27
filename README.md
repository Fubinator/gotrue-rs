## Getting Started

This is a [GoTrue](https://github.com/supabase/gotrue) client implementation in rust. Currently WIP (see the Status section for the current progress).

## Status

- [ ] Functionality
  - [x] Sign Up with Email/Phone
  - [x] Sign In with Email/Phone
  - [x] Send Magic Link Email
  - [x] Sign out
  - [ ] Verify OTP
  - [ ] Invite User by Email
  - [ ] Reset Password for Email
  - [ ] Get Url for Provider
  - [ ] Get User
  - [ ] Update User
  - [ ] Refresh Access Token
  - [ ] List Users (includes filtering, sorting, pagination)
  - [ ] Get User by Id
  - [ ] Create User
  - [ ] Update User by Id
  - [ ] Get User
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
