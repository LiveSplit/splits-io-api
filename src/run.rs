//! The run module handles retrieving Runs. A Run maps directly to an uploaded splits file.
//!
//! [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#run)

use crate::{get_json, get_response, schema::Run, wrapper::ContainsRun, Client, Error};
use reqwest::{
    header::ACCEPT,
    multipart::{Form, Part},
    Url,
};
use std::{
    io::{self, Write},
    ops::Deref,
};

impl Run {
    /// Downloads the splits for the Run.
    pub async fn download(&self, client: &Client) -> Result<impl Deref<Target = [u8]>, Error> {
        self::download(
            client,
            self.id.as_deref().ok_or(Error::UnidentifiableResource)?,
        )
        .await
    }

    /// Gets a Run.
    pub async fn get(client: &Client, id: &str, historic: bool) -> Result<Run, Error> {
        self::get(client, id, historic).await
    }

    /// Uploads a run to splits.io.
    pub async fn upload(client: &Client, run: Vec<u8>) -> Result<UploadedRun, Error> {
        self::upload(client, run).await
    }

    /// Retrieves the public URL of the run. This may fail if the run is unidentifiable.
    pub fn url(&self) -> Result<Url, Error> {
        let mut url = Url::parse("https://splits.io").unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(self.id.as_deref().ok_or(Error::UnidentifiableResource)?);
        Ok(url)
    }
}

/// Downloads the splits for a Run.
pub async fn download(client: &Client, id: &str) -> Result<impl Deref<Target = [u8]>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runs").unwrap();
    url.path_segments_mut().unwrap().push(id);

    get_response(
        client,
        client
            .client
            .get(url)
            .header(ACCEPT, "application/original-timer"),
    )
    .await?
    .bytes()
    .await
    .map_err(|source| Error::Download { source })
}

/// Gets a Run.
pub async fn get(client: &Client, id: &str, historic: bool) -> Result<Run, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runs").unwrap();
    url.path_segments_mut().unwrap().push(id);
    if historic {
        url.query_pairs_mut().append_pair("historic", "1");
    }

    let ContainsRun { run } = get_json(client, client.client.get(url)).await?;

    Ok(run)
}

#[derive(Debug, serde_derive::Deserialize)]
struct UploadResponse {
    id: Box<str>,
    claim_token: Box<str>,
    presigned_request: PresignedRequest,
}

#[derive(Debug, serde_derive::Deserialize)]
struct PresignedRequest {
    uri: Box<str>,
    fields: PresignedRequestFields,
}

#[derive(Debug, serde_derive::Deserialize, serde_derive::Serialize)]
struct PresignedRequestFields {
    key: Box<str>,
    policy: Box<str>,
    #[serde(rename = "x-amz-credential")]
    credential: Box<str>,
    #[serde(rename = "x-amz-algorithm")]
    algorithm: Box<str>,
    #[serde(rename = "x-amz-date")]
    date: Box<str>,
    #[serde(rename = "x-amz-signature")]
    signature: Box<str>,
}

/// A run that was uploaded to splits.io.
#[derive(Debug)]
pub struct UploadedRun {
    /// The unique ID for identifying the run.
    pub id: Box<str>,
    /// The token that can be used by the user to claim the run as their own.
    pub claim_token: Box<str>,
}

impl UploadedRun {
    /// Gets the uploaded run.
    pub async fn get(&self, client: &Client, historic: bool) -> Result<Run, Error> {
        Run::get(client, &self.id, historic).await
    }

    /// Retrieves the public URL of the uploaded run.
    pub fn public_url(&self) -> Url {
        let mut url = Url::parse("https://splits.io").unwrap();
        url.path_segments_mut().unwrap().push(&self.id);
        url
    }

    /// Retrieves the URL to claim the uploaded run.
    pub fn claim_url(&self) -> Url {
        let mut url = self.public_url();
        url.query_pairs_mut()
            .append_pair("claim_token", &self.claim_token);
        url
    }
}

/// Handles writing a run to the body of an upload request.
pub struct RunWriter(Vec<u8>);

impl Write for RunWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Write::write(&mut self.0, buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Uploads a run to splits.io.
pub async fn upload(client: &Client, run: Vec<u8>) -> Result<UploadedRun, Error> {
    let UploadResponse {
        id,
        claim_token,
        presigned_request: PresignedRequest { uri, fields },
    } = get_json(client, client.client.post("https://splits.io/api/v4/runs")).await?;

    get_response(
        client,
        client.client.post(String::from(uri)).multipart(
            Form::new()
                .text("key", String::from(fields.key))
                .text("policy", String::from(fields.policy))
                .text("x-amz-credential", String::from(fields.credential))
                .text("x-amz-algorithm", String::from(fields.algorithm))
                .text("x-amz-date", String::from(fields.date))
                .text("x-amz-signature", String::from(fields.signature))
                .part("file", Part::bytes(run)),
        ),
    )
    .await?;

    Ok(UploadedRun { id, claim_token })
}
