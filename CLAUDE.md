# Claude Instructions — auto-api-rust

## Language

All code comments, documentation, and README files must be written in **English**.

## Commands

- Build: `cargo build`
- Test: `cargo test`
- Run example: `cargo run --example basic`
- Lint: `cargo clippy`
- Format check: `cargo fmt --check`

## Key Files

- `src/lib.rs` — crate root, re-exports public API
- `src/client.rs` — Client struct with 6 async methods
- `src/types.rs` — all types + OffersParams for query building
- `src/error.rs` — Error enum (Auth, Api, Network variants)
- `Cargo.toml` — package manifest and dependencies

## Code Style

- Rust edition 2021
- Dependencies: reqwest (with json feature), serde, serde_json
- tokio is a dev-dependency only (used in examples and tests)
- async/await on all client methods, returning `Result<T, Error>`
- Error enum with Auth, Api, Network variants; `impl From<reqwest::Error>`
- `Option<T>` for optional parameters, `..Default::default()` pattern
- `serde_json::Value` for raw offer data (varies between sources)
- snake_case for all identifiers, pub fields on response structs
- Rustdoc (`///`) on every public item, in English
- Keep it simple — no unnecessary abstractions
