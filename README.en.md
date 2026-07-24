# Toss Invest Rust SDK (Unofficial)

**[English](README.en.md)** | [한국어](README.md)

<p align="center">
  <a href="https://github.com/epicsagas/tossinvest-sdk/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/epicsagas/tossinvest-sdk/ci.yml?style=for-the-badge&labelColor=0d1117&color=58a6ff&logo=githubactions&logoColor=white" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-3fb950?style=for-the-badge&labelColor=0d1117" /></a>
  <a href="https://github.com/epicsagas/tossinvest-sdk/stargazers"><img alt="Stars" src="https://img.shields.io/github/stars/epicsagas/tossinvest-sdk?style=for-the-badge&labelColor=0d1117&color=ffd700&logo=github&logoColor=white" /></a>
  <a href="https://github.com/epicsagas/tossinvest-sdk/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/epicsagas/tossinvest-sdk?style=for-the-badge&labelColor=0d1117&color=58a6ff&logo=git&logoColor=white" /></a>
</p>

> ⚠️ **Disclaimer**
> This library is an **Unofficial** Rust SDK, independently developed without affiliation to or endorsement by Toss Invest. The trademarks "Toss Invest" and "Toss" belong to Viva Republica / Toss Invest. See the official [Toss Invest Open API docs](https://developers.tossinvest.com/).

> 🧪 **Production Use Caution**
> This SDK has not undergone integration testing against the real Toss Invest production environment (live API). All tests run on wiremock-based mocks, so please validate thoroughly against the live environment before adopting it in production with real funds. **The developer assumes no liability for any losses arising from the use of this library.**

> 🚫 **Liability**
> The original author and the company in question (Toss Invest) assume no responsibility for any financial or material losses incurred while using this library.

> API spec: Toss Invest Open API v1.2.4 (OpenAPI 3.1.0)

An unofficial Rust SDK for the Toss Invest Open API. Built with a **hexagonal architecture (ports & adapters)** that cleanly separates domain logic from the HTTP transport layer. Covers 29 endpoints — market data, accounts/holdings, orders, and conditional orders — with a single, consistent API.

## Features

- **OAuth2 auto-auth**: Client Credentials token issuance, caching, and early refresh — injected as `Authorization: Bearer`.
- **Account header handling**: `AccountSeq` header for account-scoped endpoints is wired via `with_account()`.
- **Single error type**: every call returns one `SdkError` — init, auth, and API errors unified.
- **Domain ports**: market data · account · trading · conditional order split into separate traits.
- **Full regression tests**: wiremock-based routing, auth, and error-mapping coverage.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
tossinvest-sdk = "0.1.0"
```

## Quick Start

```rust
use tossinvest_sdk::prelude::*; // HttpClient, port traits, SdkError

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    // OAuth2 access token is auto-issued and injected into the Authorization header.
    let client = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?;

    let prices = client.get_prices(&["005930"]).await?;
    println!("{prices:?}");

    Ok(())
}
```

> Server-side only. OAuth2 Client Credentials requires a secret that must not be exposed to the browser/wasm — see [CONTRIBUTING.md](CONTRIBUTING.md).

## Authentication (OAuth2 Client Credentials)

`HttpClient::new(client_id, client_secret)` auto-issues and caches the token, refreshing at expiry (60s headroom).

```rust
let client = HttpClient::new("CLIENT_ID", "CLIENT_SECRET")?
    .with_base_url("https://openapi.tossinvest.com"); // default; override for tests/private
```

## Account Selection (AccountSeq)

Holdings · orders · conditional orders · buying power · sellable quantity · commissions require an `AccountSeq` header. Get it from `GET /accounts`, then set it:

```rust
use tossinvest_sdk::prelude::*;
use tossinvest_sdk::domain::models::Currency;

let trader = HttpClient::new("CLIENT_ID", "CLIENT_SECRET")?.with_account(123456);
let power = trader.get_buying_power(Currency::Krw).await?;
```

## Domain Ports

`HttpClient` implements 5 port traits, importable from `tossinvest_sdk::prelude`:

| Port | Covers |
| --- | --- |
| `MarketDataPort` | prices · orderbook · trades · candles · price limits · stock info · warnings · exchange rate · market calendar · rankings · market indicators · investor trading |
| `AccountPort` | accounts, holdings overview |
| `TradingPort` | orders (create/get/modify/cancel), buying power, sellable quantity, commissions |
| `ConditionalOrderPort` | conditional orders (create/get/modify/cancel) — SINGLE/OCO/OTO |
| `AuthPort` | manual OAuth2 token issuance |

## Error Handling

```rust
match client.get_prices(&["000000"]).await {
    Ok(prices) => println!("{prices:?}"),
    Err(SdkError::ApiError { status, code, message, request_id }) => {
        println!("API error {status}: {code} - {message} (request={request_id:?})");
    }
    Err(e) => eprintln!("{e}"),
}
```

| Variant | Meaning |
|---------|---------|
| `HttpError` | reqwest transport error (timeout, connection) |
| `InvalidHeaderValue` | header-incompatible characters in credentials |
| `SerializationError` | request/response (de)serialization failure |
| `ApiError { status, code, message, request_id }` | non-2xx API response |
| `Unknown` | unclassified (e.g. `AccountSeq` not set) |

## Architecture (Hexagonal / Ports & Adapters)

```
src/
├── domain/models/      # 72 data structures from the OpenAPI spec (serde)
├── port/               # domain traits (abstract boundary)
├── adapter/
│   ├── http_client.rs  # concrete port implementation (entry point)
│   ├── token.rs        # OAuth2 token issuance · caching · auto-refresh
│   └── *_adapter.rs    # HttpClient's port trait impls
└── error.rs            # single SdkError
```

Inspired by [`portone-rs`](https://github.com/portone-io/portone-rs)'s hexagonal pattern, simplified to direct reqwest+serde (no openapi-generator) for the 29-endpoint scale.

## Quality

```bash
cargo fmt                       # format
cargo clippy --all-targets -- -D warnings   # lint
cargo test                      # full suite (31 tests)
```

### Performance

Local benchmarks of the SDK hot paths (criterion):

| Benchmark | Median |
|-----------|-------:|
| Order request serialization | 128 ns |
| Prices response deserialization (50 items) | 8.50 µs |
| Prices response deserialization (200 items) | 33.47 µs |

See [docs/benchmarks.md](docs/benchmarks.md) for setup, confidence intervals, and interpretation.

### Eval Gate

The `epic eval` gate tracks correctness · performance · quality · regression · e2e — currently **OVERALL PASS (1.0)**. Latest snapshot: [docs/eval-report.md](docs/eval-report.md).

## License

[Apache-2.0](LICENSE)
