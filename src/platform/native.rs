use bytes::buf::BufExt;
use http::{Request, Response};
use hyper_tls::HttpsConnector;
use std::{io::Read, ops::Deref};

pub use hyper::{Body, Error};

pub async fn recv_bytes(body: Body) -> Result<impl Deref<Target = [u8]>, Error> {
    hyper::body::to_bytes(body).await
}

pub async fn recv_reader(body: Body) -> Result<impl Read, Error> {
    Ok(hyper::body::aggregate(body).await?.reader())
}

pub struct Client {
    client: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl Client {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        Self { client }
    }

    pub async fn request(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        self.client.request(request).await
    }
}
