# auto-api-client-rust

[![Crates.io](https://img.shields.io/crates/v/auto-api-client)](https://crates.io/crates/auto-api-client)
[![docs.rs](https://docs.rs/auto-api-client/badge.svg)](https://docs.rs/auto-api-client)
[![License](https://img.shields.io/github/license/autoapicom/auto-api-rust)](LICENSE)

Rust client for [auto-api.com](https://auto-api.com) — car listings API across multiple marketplaces.

One API to access car listings from 8 marketplaces: encar, mobile.de, autoscout24, che168, dongchedi, guazi, dubicars, dubizzle. Search offers, track price changes, and get listing data in a unified format.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
auto-api-client = "0.1"
```

## Usage

```rust
use auto_api_client::{Client, OffersParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key");
    // ...
    Ok(())
}
```

### Get filters

```rust
let filters = client.get_filters("encar").await?;
```

### Search offers

```rust
let offers = client.get_offers("mobilede", &OffersParams {
    page: 1,
    brand: Some("BMW".into()),
    year_from: Some(2020),
    ..Default::default()
}).await?;

// Pagination
println!("{}", offers.meta.page);
println!("{}", offers.meta.next_page);
```

### Get single offer

```rust
let offer = client.get_offer("encar", "40427050").await?;
```

### Track changes

```rust
let change_id = client.get_change_id("encar", "2025-01-15").await?;
let changes = client.get_changes("encar", change_id).await?;

// Next batch
let next_batch = client.get_changes("encar", changes.meta.next_change_id).await?;
```

### Get offer by URL

```rust
let info = client.get_offer_by_url(
    "https://www.encar.com/dc/dc_cardetailview.do?carid=40427050"
).await?;
```

### Decode offer data

Offer data varies between sources, so it's stored as `serde_json::Value`. Deserialize into `OfferData` or your own struct:

```rust
use auto_api_client::OfferData;

for item in &offers.result {
    let d: OfferData = serde_json::from_value(item.data.clone())?;
    println!("{} {} {} — ${}", d.mark, d.model, d.year, d.price);
}
```

### Error handling

```rust
use auto_api_client::{Client, OffersParams, Error};

match client.get_offers("encar", &OffersParams { page: 1, ..Default::default() }).await {
    Ok(offers) => println!("Got {} offers", offers.result.len()),
    Err(Error::Auth { status_code, message }) => {
        // 401/403 — invalid API key
        eprintln!("Auth error {}: {}", status_code, message);
    }
    Err(Error::Api { status_code, message, body }) => {
        // Any other API error
        eprintln!("API error {}: {}", status_code, message);
    }
    Err(Error::Network(e)) => {
        // reqwest/network error
        eprintln!("Network error: {}", e);
    }
}
```

## Supported sources

| Source | Platform | Region |
|--------|----------|--------|
| `encar` | [encar.com](https://www.encar.com) | South Korea |
| `mobilede` | [mobile.de](https://www.mobile.de) | Germany |
| `autoscout24` | [autoscout24.com](https://www.autoscout24.com) | Europe |
| `che168` | [che168.com](https://www.che168.com) | China |
| `dongchedi` | [dongchedi.com](https://www.dongchedi.com) | China |
| `guazi` | [guazi.com](https://www.guazi.com) | China |
| `dubicars` | [dubicars.com](https://www.dubicars.com) | UAE |
| `dubizzle` | [dubizzle.com](https://www.dubizzle.com) | UAE |

## Other languages

| Language | Package |
|----------|---------|
| PHP | [auto-api/client](https://github.com/autoapicom/auto-api-php) |
| TypeScript | [@auto-api/client](https://github.com/autoapicom/auto-api-node) |
| Python | [auto-api-client](https://github.com/autoapicom/auto-api-python) |
| Go | [auto-api-go](https://github.com/autoapicom/auto-api-go) |
| C# | [AutoApi.Client](https://github.com/autoapicom/auto-api-dotnet) |
| Java | [auto-api-client](https://github.com/autoapicom/auto-api-java) |
| Ruby | [auto-api-client](https://github.com/autoapicom/auto-api-ruby) |

## Documentation

[auto-api.com](https://auto-api.com)
