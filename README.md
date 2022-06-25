## Getting Started

This is a [GoTrue](https://github.com/supabase/gotrue) client implementation in rust. Currently WIP (see the Status section for the current progress).

## Status

- [ ] API
  - [ ] Sign Up with Email
  - [ ] Sign In with Email
  - [ ] Send Magic Link Email
  - [ ] Invite User by Email
  - [ ] Reset Password for Email
  - [ ] Signout
  - [ ] Get Url for Provider
  - [ ] Get User
  - [ ] Update User
  - [ ] Refresh Access Token
  - [ ] List Users (includes filtering, sorting, pagination)
  - [ ] Get User by Id
  - [ ] Create User
  - [ ] Update User by Id
- [ ] Client
  - [ ] Get User
  - [ ] Refresh Session
  - [ ] Auth State Change Handler
  - [ ] Provider Sign In (Provides URL)
- [ ] Provide Interfaces for Custom Token Persistence Functionality
- [ ] Documentation
- [ ] Unit Tests
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
