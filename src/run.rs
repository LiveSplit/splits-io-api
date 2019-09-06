use crate::{get_json, get_response, schema::Run, wrapper::ContainsRun, Client, Download, Error};
use futures::prelude::*;
use hyper::{header::CONTENT_TYPE, Body, Chunk, Request};
use snafu::ResultExt;
use std::io::{self, Write};
use url::Url;

pub async fn download(client: &Client, id: &str) -> Result<Chunk, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runs/").unwrap();
    url.path_segments_mut().unwrap().push(id);

    let response = get_response(
        client,
        Request::get(url.as_str())
            .header("Accept", "application/original-timer")
            .body(Body::empty())
            .unwrap(),
    )
    .await?;

    response.into_body().try_concat().await.context(Download)
}

pub async fn get(client: &Client, id: &str, historic: bool) -> Result<Run, Error> {
    let mut url = Url::parse("https://splits.io/api/v4/runs/").unwrap();
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

#[derive(Debug)]
pub struct UploadedRun {
    pub id: Box<str>,
    pub claim_token: Box<str>,
}

pub struct RunWriter(Vec<u8>);

impl Write for RunWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Write::write(&mut self.0, buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub async fn upload(client: &Client, run: &[u8]) -> Result<UploadedRun, Error> {
    upload_lazy(client, |writer| writer.write_all(run)).await
}

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
        Request::post("https://splits.io/api/v4/runs/")
            .body(Body::empty())
            .unwrap(),
    )
    .await?;
    // TODO: Unwrap

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
    .unwrap()
}
