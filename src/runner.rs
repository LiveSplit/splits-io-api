//! The runner module handles retrieving Runners. A Runner is a user with at least one Run tied to
//! their splits.io account.
//!
//! [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#runner)

use reqwest::Url;

use crate::{
    get_json,
    wrapper::{
        ContainsCategories, ContainsGames, ContainsPBs, ContainsRunner, ContainsRunners,
        ContainsRuns,
    },
    Category, Client, Error, Game, Run, Runner,
};

impl Runner {
    /// Searches for a Runner based on the name of the runner.
    pub async fn search(client: &Client, name: &str) -> Result<Vec<Runner>, Error> {
        self::search(client, name).await
    }

    /// Gets the Runner that is associated with the current user.
    pub async fn myself(client: &Client) -> Result<Runner, Error> {
        self::myself(client).await
    }

    /// Gets a Runner based on the name of the runner.
    pub async fn get(client: &Client, name: &str) -> Result<Runner, Error> {
        self::get(client, name).await
    }

    /// Gets the Runs that are associated with the Runner.
    pub async fn runs(&self, client: &Client) -> Result<Vec<Run>, Error> {
        get_runs(client, &self.name).await
    }

    /// Gets the personal best Runs that are associated with the Runner.
    pub async fn pbs(&self, client: &Client) -> Result<Vec<Run>, Error> {
        get_pbs(client, &self.name).await
    }

    /// Gets the Games that are associated with the Runner.
    pub async fn games(&self, client: &Client) -> Result<Vec<Game>, Error> {
        get_games(client, &self.name).await
    }

    /// Gets the Categories that are associated with the Runner.
    pub async fn categories(&self, client: &Client) -> Result<Vec<Category>, Error> {
        get_categories(client, &self.name).await
    }
}

/// Searches for a Runner based on the name of the runner.
pub async fn search(client: &Client, name: &str) -> Result<Vec<Runner>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners").unwrap();
    url.query_pairs_mut().append_pair("search", name);

    let ContainsRunners { runners } = get_json(client, client.client.get(url)).await?;

    Ok(runners)
}

/// Gets the Runner that is associated with the current user.
pub async fn myself(client: &Client) -> Result<Runner, Error> {
    let ContainsRunner { runner } =
        get_json(client, client.client.get("https://splits.io/api/v4/runner")).await?;

    Ok(runner)
}

/// Gets a Runner based on the name of the runner.
pub async fn get(client: &Client, name: &str) -> Result<Runner, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners").unwrap();
    url.path_segments_mut().unwrap().push(name);

    let ContainsRunner { runner } = get_json(client, client.client.get(url)).await?;

    Ok(runner)
}

/// Gets the Runs that are associated with a Runner.
pub async fn get_runs(client: &Client, name: &str) -> Result<Vec<Run>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners").unwrap();
    url.path_segments_mut().unwrap().extend(&[name, "runs"]);

    let ContainsRuns { runs } = get_json(client, client.client.get(url)).await?;

    Ok(runs)
}

/// Gets the personal best Runs that are associated with a Runner.
pub async fn get_pbs(client: &Client, name: &str) -> Result<Vec<Run>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners").unwrap();
    url.path_segments_mut().unwrap().extend(&[name, "pbs"]);

    let ContainsPBs { pbs } = get_json(client, client.client.get(url)).await?;

    Ok(pbs)
}

/// Gets the Games that are associated with a Runner.
pub async fn get_games(client: &Client, name: &str) -> Result<Vec<Game>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners").unwrap();
    url.path_segments_mut().unwrap().extend(&[name, "games"]);

    let ContainsGames { games } = get_json(client, client.client.get(url)).await?;

    Ok(games)
}

/// Gets the Categories that are associated with a Runner.
pub async fn get_categories(client: &Client, name: &str) -> Result<Vec<Category>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runners").unwrap();
    url.path_segments_mut()
        .unwrap()
        .extend(&[name, "categories"]);

    let ContainsCategories { categories } = get_json(client, client.client.get(url)).await?;

    Ok(categories)
}
