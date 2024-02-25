## Install instructions

// Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

// Build server
cd server
cargo build
cargo run


#### Generate TS bindings

// from server folder:
cargo test

cargo watch -- cargo run




## Database

resetting the database

sqlx migrate revert && sqlx migrate run


sqlx migrate revert
sqlx migrate run


## TODO

- [ ] Submitting survey
- [ ] login
  - [ ] email links for logging in?
  - [ ] verifying users with email
- [ ] data models for everything


stripe listen --forward-to http://localhost:8080/v1/webhook


