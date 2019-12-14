//! The game module handles retrieving Games. A Game is a collection of information about a game and
//! may contain Categories.
//!
//! [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#game)

use crate::platform::Body;
use crate::{
    get_json,
    wrapper::{ContainsCategories, ContainsGame, ContainsGames, ContainsRunners, ContainsRuns},
    Category, Client, Error, Game, Run, Runner,
};
use http::Request;
use url::Url;

/// Searches for a Game based on the name of the game.
pub async fn search(client: &Client, name: &str) -> Result<Vec<Game>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/games").unwrap();
    url.query_pairs_mut().append_pair("search", name);

    let ContainsGames { games } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(games)
}

/// Gets a Game based on the shortened title of the game.
pub async fn get(client: &Client, shortname: &str) -> Result<Game, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/games").unwrap();
    url.path_segments_mut().unwrap().push(shortname);

    let ContainsGame { game } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(game)
}

/// Gets the Categories that belong to a Game based on the shortened title of the game.
pub async fn get_categories(client: &Client, shortname: &str) -> Result<Vec<Category>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/games").unwrap();
    url.path_segments_mut()
        .unwrap()
        .extend(&[shortname, "categories"]);

    let ContainsCategories { categories } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(categories)
}

/// Gets the Runs that belong to a Game based on the shortened title of the game.
pub async fn get_runs(client: &Client, shortname: &str) -> Result<Vec<Run>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/games").unwrap();
    url.path_segments_mut()
        .unwrap()
        .extend(&[shortname, "runs"]);

    let ContainsRuns { runs } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(runs)
}

/// Gets the Runners that belong to a Game  based on the shortened title of the game.
pub async fn get_runners(client: &Client, shortname: &str) -> Result<Vec<Runner>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/games").unwrap();
    url.path_segments_mut()
        .unwrap()
        .extend(&[shortname, "runners"]);

    let ContainsRunners { runners } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(runners)
}
