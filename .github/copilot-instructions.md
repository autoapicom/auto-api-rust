# GitHub Copilot Instructions — auto-api-rust

Rust client library for [auto-api.com](https://auto-api.com).

## Overview

- `Client` struct with 6 async methods for interacting with the Auto API
- Dependencies: reqwest (with json feature), serde, serde_json
- tokio is a dev-dependency only (for examples and tests)
- Error enum with Auth, Api, Network variants for structured error handling
- All methods return `Result<T, Error>`
- `serde_json::Value` for raw offer data that varies between sources

## Guidelines

- All code comments and documentation must be in English
- Rustdoc (`///`) on every public item
- Use `Option<T>` for optional parameters
- Use `..Default::default()` pattern for building params
- Never use `unwrap()` in library code
- Never add tokio as a main dependency
- Never use `panic!` in library code
