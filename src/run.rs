//! The run module handles retrieving Runs. A Run maps directly to an uploaded splits file.
//!
//! [API Documentation](https://github.com/glacials/splits-io/blob/master/docs/api.md#run)

use crate::{
    get_json, get_response,
    platform::{recv_bytes, Body},
    schema::Run,
    wrapper::ContainsRun,
    Client, DownloadSnafu, Error, UnidentifiableResourceSnafu,
};
use http::{header::CONTENT_TYPE, Request};
use snafu::{OptionExt, ResultExt};
use std::{
    io::{self, Write},
    ops::Deref,
};
use url::Url;

impl Run {
    /// Downloads the splits for the Run.
    pub async fn download(&self, client: &Client) -> Result<impl Deref<Target = [u8]>, Error> {
        self::download(
            client,
            self.id.as_ref().context(UnidentifiableResourceSnafu)?,
        )
        .await
    }

    /// Gets a Run.
    pub async fn get(client: &Client, id: &str, historic: bool) -> Result<Run, Error> {
        self::get(client, id, historic).await
    }

    /// Uploads a run to Splits.io.
    pub async fn upload(client: &Client, run: &[u8]) -> Result<UploadedRun, Error> {
        self::upload(client, run).await
    }

    /// Uploads a run to Splits.io by using a RunWriter in order to write the request body.
    pub async fn upload_lazy<E: std::error::Error>(
        client: &Client,
        write_run: impl FnOnce(&mut RunWriter) -> Result<(), E>,
    ) -> Result<UploadedRun, Error> {
        self::upload_lazy(client, write_run).await
    }

    /// Retrieves the public URL of the run. This may fail if the run is unidentifiable.
    pub fn url(&self) -> Result<Url, Error> {
        let mut url = Url::parse("https://splits.io").unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(self.id.as_ref().context(UnidentifiableResourceSnafu)?);
        Ok(url)
    }
}

/// Downloads the splits for a Run.
pub async fn download(client: &Client, id: &str) -> Result<impl Deref<Target = [u8]>, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runs").unwrap();
    url.path_segments_mut().unwrap().push(id);

    let response = get_response(
        client,
        Request::get(url.as_str())
            .header("Accept", "application/original-timer")
            .body(Body::empty())
            .unwrap(),
    )
    .await?;

    recv_bytes(response.into_body())
        .await
        .context(DownloadSnafu)
}

/// Gets a Run.
pub async fn get(client: &Client, id: &str, historic: bool) -> Result<Run, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runs").unwrap();
    url.path_segments_mut().unwrap().push(id);
    if historic {
        url.query_pairs_mut().append_pair("historic", "1");
    }

    let ContainsRun { run } = get_json(
        client,
        Request::get(url.as_str()).body(Body::empty()).unwrap(),
    )
    .await?;

    Ok(run)
}

#[derive(Debug, serde::Deserialize)]
struct UploadResponse {
    id: Box<str>,
    claim_token: Box<str>,
    presigned_request: PresignedRequest,
}

#[derive(Debug, serde::Deserialize)]
struct PresignedRequest {
    uri: Box<str>,
    fields: PresignedRequestFields,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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

/// A run that was uploaded to Splits.io.
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

/// Uploads a run to Splits.io.
pub async fn upload(client: &Client, run: &[u8]) -> Result<UploadedRun, Error> {
    upload_lazy(client, |writer| writer.write_all(run)).await
}

/// Uploads a run to Splits.io by using a RunWriter in order to write the request body.
pub async fn upload_lazy<E: std::error::Error>(
    client: &Client,
    write_run: impl FnOnce(&mut RunWriter) -> Result<(), E>,
) -> Result<UploadedRun, Error> {
    let UploadResponse {
        id,
        claim_token,
        presigned_request: PresignedRequest { uri, fields },
    } = get_json(
        client,
        Request::post("https://splits.io/api/v4/runs")
            .body(Body::empty())
            .unwrap(),
    )
    .await?;
    // FIXME: Unwrap

    let mut body = Vec::new();

    write_key_and_value(&mut body, "key", &fields.key);
    write_key_and_value(&mut body, "policy", &fields.policy);
    write_key_and_value(&mut body, "x-amz-credential", &fields.credential);
    write_key_and_value(&mut body, "x-amz-algorithm", &fields.algorithm);
    write_key_and_value(&mut body, "x-amz-date", &fields.date);
    write_key_and_value(&mut body, "x-amz-signature", &fields.signature);

    write!(
        &mut body,
        "------WebKitFormBoundarymfBzYhzpfnJqay4s\r\nContent-Disposition: form-data; name=\"file\"\r\n\r\n"
    )
    .unwrap();
    let mut writer = RunWriter(body);
    write_run(&mut writer).unwrap();

    let mut body = writer.0;
    write!(
        &mut body,
        "\r\n------WebKitFormBoundarymfBzYhzpfnJqay4s--\r\n"
    )
    .unwrap();

    get_response(
        client,
        Request::post(&*uri)
            .header(
                CONTENT_TYPE,
                "multipart/form-data; boundary=----WebKitFormBoundarymfBzYhzpfnJqay4s",
            )
            .body(Body::from(body))
            .unwrap(),
    )
    .await?;

    Ok(UploadedRun { id, claim_token })
}

fn write_key_and_value(w: &mut impl Write, key: &str, value: &str) {
    write!(
        w,
        "------WebKitFormBoundarymfBzYhzpfnJqay4s\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
        key, value,
    )
    .unwrap();
}
