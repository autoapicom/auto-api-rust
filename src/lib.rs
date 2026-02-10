//! # auto-api-client
//!
//! Rust client for [auto-api.com](https://auto-api.com) — car listings API
//! across multiple marketplaces.
//!
//! ## Usage
//!
//! ```no_run
//! use auto_api_client::{Client, OffersParams};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("your-api-key");
//!
//!     let offers = client.get_offers("encar", &OffersParams {
//!         page: 1,
//!         brand: Some("BMW".into()),
//!         ..Default::default()
//!     }).await?;
//!
//!     println!("Got {} offers", offers.result.len());
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod types;

pub use client::Client;
pub use error::Error;
pub use types::*;
