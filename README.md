# Zero to Production in Rust

My learning project following _Zero to Production in Rust_ by Luca Palmieri.
This repository tracks my progress through the book as I build a Rust web
application, write integration tests, and set up continuous integration.

## Current progress

- HTTP server built with Actix Web and Tokio.
- `GET /health_check` endpoint returning `200 OK` with an empty body.
- Integration test using Reqwest and a server running on a random local port.
- GitHub Actions for formatting, linting, tests, documentation, and security auditing.

The application is a work in progress and will grow as I work through the book.

## Run locally

Install Rust and Cargo using rustup, then run these commands from the repository root:

```sh
cargo run --locked
```

The server listens on `127.0.0.1:8000`. In another terminal, check the endpoint:

```sh
curl -i http://127.0.0.1:8000/health_check
```

## Tests and checks

```sh
cargo test --all-features --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps --all-features --locked
```

To scan dependencies for known security advisories:

```sh
cargo install cargo-audit --locked
cargo audit
```

## Git hooks

Install [Lefthook](https://lefthook.dev/installation/) once, then enable the
hooks in each clone. On macOS:

```sh
brew install lefthook
lefthook install
```

Before each commit, Lefthook runs `cargo fmt --check`. Before each push, it
runs Clippy and the tests sequentially using the commands above. These checks
run against the working tree, so unstaged Rust changes can also cause failures.
Formatting is checked without modifying files; run `cargo fmt` to fix failures,
then review and stage the changes.

To run the hooks manually:

```sh
lefthook run pre-commit --force
lefthook run pre-push --force
```

The configuration lives in `lefthook.yml`. CI continues to run independently
of local hooks.

## Project layout

```text
src/main.rs               Application entry point and listener setup
src/lib.rs                Server construction and health-check route
tests/health_check.rs     HTTP integration test
.github/workflows/ci.yml  Continuous integration workflow
```
