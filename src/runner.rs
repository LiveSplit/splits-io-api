use crate::{
    get_json,
    wrapper::{
        ContainsCategories, ContainsGames, ContainsPBs, ContainsRunner, ContainsRunners,
        ContainsRuns,
    },
    Category, Client, Error, Game, Run, Runner,
};
use hyper::{Body, Request};
use url::Url;

pub async fn search(client: &Client, name: &str) -> Result<Vec<Runner>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners").unwrap();
    url.query_pairs_mut().append_pair("search", name);

    let ContainsRunners { runners } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(runners)
}

pub async fn get(client: &Client, name: &str) -> Result<Runner, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners/").unwrap();
    url.path_segments_mut().unwrap().push(name);

    let ContainsRunner { runner } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(runner)
}

pub async fn get_runs(client: &Client, name: &str) -> Result<Vec<Run>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners/").unwrap();
    url.path_segments_mut().unwrap().extend(&[name, "runs"]);

    let ContainsRuns { runs } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(runs)
}

pub async fn get_pbs(client: &Client, name: &str) -> Result<Vec<Run>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners/").unwrap();
    url.path_segments_mut().unwrap().extend(&[name, "pbs"]);

    let ContainsPBs { pbs } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(pbs)
}

pub async fn get_games(client: &Client, name: &str) -> Result<Vec<Game>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners/").unwrap();
    url.path_segments_mut().unwrap().extend(&[name, "games"]);

    let ContainsGames { games } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(games)
}

pub async fn get_categories(client: &Client, name: &str) -> Result<Vec<Category>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners/").unwrap();
    url.path_segments_mut()
        .unwrap()
        .extend(&[name, "categories"]);

    let ContainsCategories { categories } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(categories)
}
