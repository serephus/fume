//! Backend-agnostic HTTP transport.
//!
//! The transport traits are the seam between `fume` and whatever performs the
//! actual network I/O. Because a request is a plain value ([`HttpRequest`]) and a
//! response is a plain value ([`HttpResponse`]), a backend only has to move
//! bytes — all URL building, JSON decoding and error interpretation happens in
//! `fume` itself.

use std::future::Future;

use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Method, StatusCode, header};

/// A fully-built HTTP request, ready to be handed to a transport.
#[derive(Clone, Debug)]
pub struct HttpRequest {
    /// HTTP verb.
    pub method: Method,
    /// Absolute request URL, including percent-encoded query.
    pub url: String,
    /// Request headers.
    pub headers: HeaderMap,
    /// Raw request body. Empty for bodyless requests.
    pub body: Bytes,
}

impl HttpRequest {
    /// Create a request with the given method and URL.
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    /// Create a `GET` request.
    pub fn get(url: impl Into<String>) -> Self {
        Self::new(Method::GET, url)
    }

    /// Create a `POST` request.
    pub fn post(url: impl Into<String>) -> Self {
        Self::new(Method::POST, url)
    }

    /// Insert a header.
    #[must_use]
    pub fn header(mut self, name: impl header::IntoHeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Set the request body.
    #[must_use]
    pub fn body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = body.into();
        self
    }
}

/// A raw HTTP response returned by a transport.
#[derive(Clone, Debug)]
pub struct HttpResponse {
    /// HTTP status code.
    pub status: StatusCode,
    /// Response headers.
    pub headers: HeaderMap,
    /// Raw response body.
    pub body: Bytes,
}

impl HttpResponse {
    /// Create a response from a status and body.
    pub fn new(status: StatusCode, body: impl Into<Bytes>) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body: body.into(),
        }
    }

    /// Decode the body as UTF-8, replacing invalid sequences.
    pub fn text_lossy(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(&self.body)
    }
}

/// An asynchronous HTTP transport.
///
/// Implement this for a backend that performs non-blocking I/O, such as
/// `reqwest`. The returned future must be `Send` so clients can be driven from
/// work-stealing runtimes.
pub trait AsyncTransport: Clone + Send + Sync + 'static {
    /// The backend's error type.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Execute a request.
    fn execute(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> + Send;
}

/// A blocking HTTP transport.
///
/// Implement this for a backend that performs blocking I/O, such as
/// `reqwest::blocking` or `ureq`. No async runtime is required.
pub trait BlockingTransport: Clone + Send + Sync + 'static {
    /// The backend's error type.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Execute a request.
    fn execute(&self, request: HttpRequest) -> Result<HttpResponse, Self::Error>;
}
