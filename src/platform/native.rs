use http::{Request, Response};
use hyper_rustls::HttpsConnector;

pub use hyper::{Body, Chunk, Error};

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
