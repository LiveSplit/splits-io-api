use http::{
    header::{HeaderName, HeaderValue},
    request::Parts,
    Request, Response, StatusCode,
};
use js_sys::{Array, Reflect, Uint8Array};
use snafu::OptionExt;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, RequestInit};

pub struct Client;

#[derive(Default)]
pub struct Chunk(Vec<u8>);

impl IntoIterator for Chunk {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::iter::Extend<u8> for Chunk {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        self.0.extend(iter)
    }
}

impl std::ops::Deref for Chunk {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &*self.0
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// There is no global window object to be used.
    NoWindow,
    /// A forbidden header was used.
    #[snafu(display("A forbidden header was used: {}", error.as_string().unwrap_or_default()))]
    ForbiddenHeader { error: JsValue },
    /// Failed to receive the response.
    #[snafu(display("Failed to receive the response: {}", error.as_string().unwrap_or_default()))]
    ReceiveResponse { error: JsValue },
}

pub struct Body {
    data: Option<Vec<u8>>,
}

impl From<Vec<u8>> for Body {
    fn from(data: Vec<u8>) -> Body {
        Body { data: Some(data) }
    }
}

impl Client {
    pub fn new() -> Self {
        Client
    }

    pub async fn request(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        let window = window().context(NoWindow)?;

        let (
            Parts {
                method,
                uri,
                version: _,
                headers,
                extensions: _,
                ..
            },
            body,
        ) = request.into_parts();

        let mut request_init = RequestInit::new();

        request_init.method(method.as_str());

        if let Some(body) = &body.data {
            let view = unsafe { Uint8Array::view(&body) };
            request_init.body(Some(view.unchecked_ref()));
        }

        let request_headers = web_sys::Headers::new().unwrap();

        for (name, value) in &headers {
            request_headers
                .append(name.as_str(), value.to_str().unwrap_or(""))
                .map_err(|error| Error::ForbiddenHeader { error })?;
        }

        request_init.headers(request_headers.unchecked_ref());

        let web_response: web_sys::Response =
            JsFuture::from(window.fetch_with_str_and_init(&uri.to_string(), &request_init))
                .await
                .map_err(|error| Error::ReceiveResponse { error })?
                .unchecked_into();

        // Don't drop this earlier, we unsafely borrow from it for the request.
        drop(body);

        let buf: js_sys::ArrayBuffer = JsFuture::from(
            web_response
                .array_buffer()
                .map_err(|error| Error::ReceiveResponse { error })?,
        )
        .await
        .map_err(|error| Error::ReceiveResponse { error })?
        .unchecked_into();

        let slice = Uint8Array::new(&buf);
        let mut body: Vec<u8> = vec![0; slice.length() as usize];
        slice.copy_to(&mut body);

        let mut response = Response::new(Body { data: Some(body) });

        *response.status_mut() = StatusCode::from_u16(web_response.status()).unwrap();

        let headers = response.headers_mut();

        let prop = String::from("value").into();

        for pair in js_sys::try_iter(&web_response.headers()).unwrap().unwrap() {
            let array: Array = pair.unwrap().into();
            let vals = array.values();

            let key = Reflect::get(&vals.next().unwrap(), &prop).unwrap();
            let value = Reflect::get(&vals.next().unwrap(), &prop).unwrap();

            let key = key.as_string().unwrap();
            let value = value.as_string().unwrap();

            headers.append(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        Ok(response)
    }
}

impl Body {
    pub fn empty() -> Self {
        Self { data: None }
    }
}

use core::{
    pin::Pin,
    task::{Context, Poll},
};

impl futures_util::stream::Stream for Body {
    type Item = Result<Chunk, Error>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.data.take().map(|v| Ok(Chunk(v))))
    }
}
