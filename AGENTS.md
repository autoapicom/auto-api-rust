# auto-api Rust Client

Rust client for [auto-api.com](https://auto-api.com) — car listings API across 8 marketplaces.

## Quick Start

```toml
[dependencies]
auto-api-client = "1.0"
```

```rust
use auto_api_client::{Client, OffersParams};

let client = Client::new("your-api-key");
let offers = client.get_offers("encar", &OffersParams { page: 1, ..Default::default() }).await?;
```

## Build & Test

```bash
cargo build
cargo test
cargo run --example basic
cargo clippy
cargo fmt --check
```

## Key Files

- `src/lib.rs` — Public re-exports, module declarations, crate-level docs
- `src/client.rs` — Client struct, new(), 6 async methods, HTTP helpers
- `src/types.rs` — OffersParams, OffersResponse, ChangesResponse, Meta, OfferItem, OfferData
- `src/error.rs` — Error enum with Auth, Api, Network variants
- `Cargo.toml` — Crate metadata, dependencies (reqwest, serde, serde_json)

## Conventions

- Rust edition 2021, dependencies: reqwest (json), serde, serde_json
- tokio is dev-dependency only (for examples)
- async/await — all public methods return Result<T, Error>
- Error enum with Auth, Api, Network variants — exhaustive pattern matching
- serde_json::Value for raw JSON data — structure varies between sources
- Option<String> for optional parameters, ..Default::default() for partial init
- snake_case for everything — Rust convention
- pub fields on response structs — Rust convention for data types
- Rustdoc comments (///) on every public type and method, in English
- impl From<reqwest::Error> — automatic error conversion with ? operator

## API Methods

| Method | Params | Returns |
|--------|--------|---------|
| `get_filters(source)` | &str | `Result<Value, Error>` |
| `get_offers(source, params)` | &str + &OffersParams | `Result<OffersResponse, Error>` |
| `get_offer(source, inner_id)` | &str + &str | `Result<OffersResponse, Error>` |
| `get_change_id(source, date)` | &str + &str | `Result<i64, Error>` |
| `get_changes(source, change_id)` | &str + i64 | `Result<ChangesResponse, Error>` |
| `get_offer_by_url(url)` | &str | `Result<Value, Error>` |
