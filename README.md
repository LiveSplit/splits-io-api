# splits-io-api

[![Build Status](https://github.com/LiveSplit/splits-io-api/workflows/Rust/badge.svg)](https://github.com/LiveSplit/splits-io-api/actions)

Bindings to the Splits.io API for Rust. Both native platforms and the web are
supported.

## Example Usage

```rust
// Create a Splits.io API client.
let client = Client::new();

// Search for a runner.
let runners = runner::search(&client, "cryze").await.unwrap();
let runner = runners.first().unwrap();
let runner_name = &*runner.name;
assert_eq!(runner_name, "cryze92");

// Get the PBs for the runner.
let runner_pbs = runner::get_pbs(&client, runner_name).await.unwrap();
let first_pb = &*runner_pbs.first().unwrap();

// Get the game for the PB.
let pb_game = first_pb.game.as_ref().unwrap();
let pb_game_shortname = pb_game.shortname.as_ref().unwrap();
assert_eq!(pb_game_shortname.as_ref(), "tww");

// Get the categories for the game.
let game_categories = game::get_categories(&client, pb_game_shortname).await.unwrap();

// Get the runs for the Any% category.
let any_percent = game_categories.iter().find(|category| &*category.name == "Any%").unwrap();
let any_percent_runs = category::get_runs(&client, &any_percent.id).await.unwrap();
assert!(!any_percent_runs.is_empty());
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
