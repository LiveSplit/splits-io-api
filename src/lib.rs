#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    missing_docs,
    rust_2018_idioms
)]

//! splits-io-api is a library that provides bindings for the Splits.io API for Rust.

use crate::platform::{recv_reader, Body};
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

/// A client that can access the Splits.io API. This includes an access token that is used for
/// authentication to all API endpoints.
pub struct Client {
    client: platform::Client,
    access_token: Option<String>,
}

impl Client {
    /// Creates a new client.
    pub fn new() -> Self {
        Client {
            client: platform::Client::new(),
            access_token: None,
        }
    }

    /// Sets the client's access token, which can be used to authenticate to all API endpoints.
    pub fn set_access_token(&mut self, access_token: &str) {
        let buf = self.access_token.get_or_insert_with(String::new);
        buf.clear();
        buf.push_str("Bearer ");
        buf.push_str(access_token);
    }
}

#[derive(Debug, snafu::Snafu)]
/// An error when making an API request.
pub enum Error {
    /// An HTTP error outside of the API.
    #[snafu(display("HTTP Status Code: {}", status.canonical_reason().unwrap_or_else(|| status.as_str())))]
    Status {
        /// The HTTP status code of the error.
        status: StatusCode,
    },
    /// An error thrown by the API.
    #[snafu(display("{}", message))]
    Api {
        /// The HTTP status code of the error.
        status: StatusCode,
        /// The error message.
        message: Box<str>,
    },
    /// Failed downloading the response.
    Download {
        /// The lower-level source of the error.
        source: crate::platform::Error,
    },
    /// Failed to parse the response.
    Json {
        /// The lower-level source of the error.
        source: serde_json::Error,
    },
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
        if let Ok(reader) = recv_reader(response.into_body()).await {
            if let Ok(error) = serde_json::from_reader::<_, ApiError>(reader) {
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
    let reader = recv_reader(response.into_body()).await.context(Download)?;
    serde_json::from_reader(reader).context(Json)
}

#[derive(serde::Deserialize)]
struct ApiError {
    #[serde(alias = "message")]
    error: Box<str>,
}
