use crate::platform::Body;
use crate::{
    get_json,
    wrapper::{ContainsCategory, ContainsRunners, ContainsRuns},
    Category, Client, Error, Run, Runner,
};
use http::Request;
use url::Url;

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
