#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::undocumented_unsafe_blocks,
    missing_docs,
    rust_2018_idioms
)]

//! splits-io-api is a library that provides bindings for the splits.io API for Rust.
//!
//! ```no_run
//! # use splits_io_api::{Client, Runner};
//! # use anyhow::Context;
//! #
//! # async fn query_api() -> anyhow::Result<()> {
//! // Create a splits.io API client.
//! let client = Client::new();
//!
//! // Search for a runner.
//! let runner = Runner::search(&client, "cryze")
//!     .await?
//!     .into_iter()
//!     .next()
//!     .context("There is no runner with that name")?;
//!
//! assert_eq!(&*runner.name, "cryze92");
//!
//! // Get the PBs for the runner.
//! let first_pb = runner.pbs(&client)
//!     .await?
//!     .into_iter()
//!     .next()
//!     .context("This runner doesn't have any PBs")?;
//!
//! // Get the game for the PB.
//! let game = first_pb.game.context("There is no game for the PB")?;
//!
//! assert_eq!(&*game.name, "The Legend of Zelda: The Wind Waker");
//!
//! // Get the categories for the game.
//! let categories = game.categories(&client).await?;
//!
//! // Get the runs for the Any% category.
//! let runs = categories
//!     .iter()
//!     .find(|category| &*category.name == "Any%")
//!     .context("Couldn't find category")?
//!     .runs(&client)
//!     .await?;
//!
//! assert!(!runs.is_empty());
//! # Ok(())
//! # }
//! ```

use std::fmt;

use reqwest::{header::AUTHORIZATION, RequestBuilder, Response, StatusCode};

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

/// A client that can access the splits.io API. This includes an access token that is used for
/// authentication to all API endpoints.
pub struct Client {
    client: reqwest::Client,
    access_token: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut builder = reqwest::Client::builder();
        #[cfg(not(target_family = "wasm"))]
        {
            builder = builder.http2_prior_knowledge();
            #[cfg(feature = "rustls")]
            {
                builder = builder.use_rustls_tls();
            }
        }

        Client {
            client: builder.build().unwrap(),
            access_token: None,
        }
    }
}

impl Client {
    /// Creates a new client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the client's access token, which can be used to authenticate to all API endpoints.
    pub fn set_access_token(&mut self, access_token: &str) {
        let buf = self.access_token.get_or_insert_with(String::new);
        buf.clear();
        buf.push_str("Bearer ");
        buf.push_str(access_token);
    }
}

#[derive(Debug)]
/// An error when making an API request.
pub enum Error {
    /// An HTTP error outside of the API.
    Status {
        /// The HTTP status code of the error.
        status: StatusCode,
    },
    /// An error thrown by the API.
    Api {
        /// The HTTP status code of the error.
        status: StatusCode,
        /// The error message.
        message: Box<str>,
    },
    /// Failed downloading the response.
    Download {
        /// The reason why downloading the response failed.
        source: reqwest::Error,
    },
    /// The resource can not be sufficiently identified for finding resources
    /// attached to it.
    UnidentifiableResource,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Status { status } => {
                write!(
                    fmt,
                    "HTTP Status: {}",
                    status.canonical_reason().unwrap_or_else(|| status.as_str()),
                )
            }
            Error::Api { message, .. } => fmt::Display::fmt(message, fmt),
            Error::Download { .. } => {
                fmt::Display::fmt("Failed downloading the response.", fmt)
            }
            Error::UnidentifiableResource => {
                fmt::Display::fmt(
                    "The resource can not be sufficiently identified for finding resources attached to it.",
                    fmt,
                )
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Status { .. } => None,
            Error::Api { .. } => None,
            Error::Download { source, .. } => Some(source),
            Error::UnidentifiableResource => None,
        }
    }
}

async fn get_response_unchecked(
    client: &Client,
    mut request: RequestBuilder,
) -> Result<Response, Error> {
    // FIXME: Only for requests that need it.
    if let Some(access_token) = &client.access_token {
        request = request.header(AUTHORIZATION, access_token);
    }

    request
        .send()
        .await
        .map_err(|source| Error::Download { source })
}

async fn get_response(client: &Client, request: RequestBuilder) -> Result<Response, Error> {
    let response = get_response_unchecked(client, request).await?;
    let status = response.status();
    if !status.is_success() {
        if let Ok(error) = response.json::<ApiError>().await {
            return Err(Error::Api {
                status,
                message: error.error,
            });
        }
        return Err(Error::Status { status });
    }
    Ok(response)
}

async fn get_json<T: serde::de::DeserializeOwned>(
    client: &Client,
    request: RequestBuilder,
) -> Result<T, Error> {
    let response = get_response(client, request).await?;
    response
        .json()
        .await
        .map_err(|source| Error::Download { source })
}

#[derive(serde_derive::Deserialize)]
struct ApiError {
    #[serde(alias = "message")]
    error: Box<str>,
}
