# <img src="https://raw.githubusercontent.com/glacials/splits-io/dec549110968c5a02df87cddff49a43549cceb92/public/logo.png" alt="splits.io" height="42" width="42" align="top"/> splits-io-api

[![Build Status](https://github.com/LiveSplit/splits-io-api/workflows/Rust/badge.svg)](https://github.com/LiveSplit/splits-io-api/actions)
[![crates.io](https://img.shields.io/crates/v/splits-io-api.svg)](https://crates.io/crates/splits-io-api)
[![docs.rs](https://docs.rs/splits-io-api/badge.svg)](https://docs.rs/splits-io-api/)
[![dependency status](https://deps.rs/repo/github/LiveSplit/splits-io-api/status.svg)](https://deps.rs/repo/github/LiveSplit/splits-io-api)

Bindings to the splits.io API for Rust. Both native platforms and the web are
supported.

## Example Usage

```rust
// Create a splits.io API client.
let client = Client::new();

// Search for a runner.
let runner = Runner::search(&client, "cryze")
    .await?
    .into_iter()
    .next()
    .context("There is no runner with that name")?;

assert_eq!(&*runner.name, "cryze92");

// Get the PBs for the runner.
let first_pb = runner.pbs(&client)
    .await?
    .into_iter()
    .next()
    .context("This runner doesn't have any PBs")?;

// Get the game for the PB.
let game = first_pb.game.context("There is no game for the PB")?;

assert_eq!(&*game.name, "The Legend of Zelda: The Wind Waker");

// Get the categories for the game.
let categories = game.categories(&client).await?;

// Get the runs for the Any% category.
let runs = categories
    .iter()
    .find(|category| &*category.name == "Any%")
    .context("Couldn't find category")?
    .runs(&client)
    .await?;

assert!(!runs.is_empty());
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
