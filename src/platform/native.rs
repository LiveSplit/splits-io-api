use http::{Request, Response};
use hyper::body::Buf;

#[cfg(all(
    any(target_os = "linux", target_family = "windows", target_os = "macos"),
    any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
    ),
))]
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
#[cfg(not(all(
    any(target_os = "linux", target_family = "windows", target_os = "macos"),
    any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
    ),
)))]
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
        #[cfg(all(
            any(target_os = "linux", target_family = "windows", target_os = "macos"),
            any(
                target_arch = "x86",
                target_arch = "x86_64",
                target_arch = "arm",
                target_arch = "aarch64",
            ),
        ))]
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        #[cfg(not(all(
            any(target_os = "linux", target_family = "windows", target_os = "macos"),
            any(
                target_arch = "x86",
                target_arch = "x86_64",
                target_arch = "arm",
                target_arch = "aarch64",
            ),
        )))]
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        Self { client }
    }

    pub async fn request(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        self.client.request(request).await
    }
}
