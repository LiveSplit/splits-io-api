//! The category module handles retrieving Categories. A Category is a ruleset for a Game and may
//! contain Runs.
//!
//! [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#category)

use crate::{
    get_json,
    platform::Body,
    wrapper::{ContainsCategory, ContainsRunners, ContainsRuns},
    Category, Client, Error, Run, Runner,
};
use http::Request;
use url::Url;

impl Category {
    /// Gets a Category.
    pub async fn get(client: &Client, id: &str) -> Result<Self, Error> {
        self::get(client, id).await
    }

    /// Gets the Runners that belong to the Category.
    pub async fn runners(&self, client: &Client) -> Result<Vec<Runner>, Error> {
        get_runners(client, &self.id).await
    }

    /// Gets the Runs that belong to the Category.
    pub async fn runs(&self, client: &Client) -> Result<Vec<Run>, Error> {
        get_runs(client, &self.id).await
    }
}

/// Gets a Category.
pub async fn get(client: &Client, id: &str) -> Result<Category, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/categories").unwrap();
    url.path_segments_mut().unwrap().push(id);

    let ContainsCategory { category } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(category)
}

/// Gets the Runners that belong to a Category.
pub async fn get_runners(client: &Client, id: &str) -> Result<Vec<Runner>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/categories").unwrap();
    url.path_segments_mut().unwrap().extend(&[id, "runners"]);

    let ContainsRunners { runners } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(runners)
}

/// Gets the Runs that belong to a Category.
pub async fn get_runs(client: &Client, id: &str) -> Result<Vec<Run>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/categories").unwrap();
    url.path_segments_mut().unwrap().extend(&[id, "runs"]);

    let ContainsRuns { runs } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(runs)
}
