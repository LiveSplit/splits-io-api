use crate::platform::Body;
use futures_util::try_stream::TryStreamExt;
use http::{header::AUTHORIZATION, Request, Response, StatusCode};
use snafu::ResultExt;

mod platform;

pub mod category;
// pub mod event;
pub mod game;
pub mod race;
pub mod run;
pub mod runner;
mod schema;
mod wrapper;
pub use schema::*;

pub use uuid;

pub struct Client {
    client: platform::Client,
    access_token: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: platform::Client::new(),
            access_token: None,
        }
    }

    pub fn set_access_token(&mut self, access_token: &str) {
        let buf = self.access_token.get_or_insert_with(String::new);
        buf.clear();
        buf.push_str("Bearer ");
        buf.push_str(access_token);
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("HTTP Status Code: {}", status.canonical_reason().unwrap_or_else(|| status.as_str())))]
    Status { status: StatusCode },
    #[snafu(display("{}", message))]
    Api {
        status: StatusCode,
        message: Box<str>,
    },
    /// Failed downloading the response.
    Download { source: crate::platform::Error },
    /// Failed to parse the response.
    Json { source: serde_json::Error },
}

async fn get_response_unchecked(
    client: &Client,
    mut request: Request<Body>,
) -> Result<Response<Body>, Error> {
    // TODO: Only for requests that need it.
    if let Some(access_token) = &client.access_token {
        // TODO: Don't ignore error.
        if let Ok(access_token) = access_token.parse() {
            request.headers_mut().insert(AUTHORIZATION, access_token);
        }
    }

    let response = client.client.request(request).await.context(Download)?;
    Ok(response)
}

async fn get_response(client: &Client, request: Request<Body>) -> Result<Response<Body>, Error> {
    let response = get_response_unchecked(client, request).await?;
    let status = response.status();
    if !status.is_success() {
        if let Ok(buf) = response.into_body().try_concat().await {
            if let Ok(error) = serde_json::from_reader::<_, ApiError>(&*buf) {
                return Err(Error::Api {
                    status,
                    message: error.error,
                });
            }
        }
        return Err(Error::Status { status });
    }
    Ok(response)
}

async fn get_json<T: serde::de::DeserializeOwned>(
    client: &Client,
    request: Request<Body>,
) -> Result<T, Error> {
    let response = get_response(client, request).await?;
    let buf = response.into_body().try_concat().await.context(Download)?;
    println!("{}", String::from_utf8_lossy(&*buf));
    serde_json::from_reader(&*buf).context(Json)
}

#[derive(serde::Deserialize)]
struct ApiError {
    #[serde(alias = "message")]
    error: Box<str>,
}
