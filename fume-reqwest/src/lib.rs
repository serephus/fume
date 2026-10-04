//! [`reqwest`] backends for [`fume`].
//!
//! This crate provides two transports:
//!
//! * [`ReqwestBackend`] — an asynchronous [`AsyncTransport`] over
//!   [`reqwest::Client`], enabled by the `async` feature (default).
//! * [`ReqwestBlockingBackend`] — a blocking [`BlockingTransport`] over
//!   [`reqwest::blocking::Client`], enabled by the `blocking` feature.
//!
//! ```rust,no_run
//! use fume::{ApiKey, Client};
//! use fume_reqwest::ReqwestBackend;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(ReqwestBackend::new(), ApiKey::new("KEY"));
//! let interfaces = client.apis().await?;
//! println!("{} interfaces", interfaces.len());
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "async")]
use fume::AsyncTransport;
#[cfg(feature = "blocking")]
use fume::BlockingTransport;
#[cfg(any(feature = "async", feature = "blocking"))]
use fume::{HttpRequest, HttpResponse};

/// An asynchronous transport backed by [`reqwest::Client`].
#[cfg(feature = "async")]
#[derive(Clone, Debug, Default)]
pub struct ReqwestBackend {
    client: reqwest::Client,
}

#[cfg(feature = "async")]
impl ReqwestBackend {
    /// Create a backend with a default [`reqwest::Client`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a backend from an existing [`reqwest::Client`].
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// The underlying [`reqwest::Client`].
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

#[cfg(feature = "async")]
impl AsyncTransport for ReqwestBackend {
    type Error = reqwest::Error;

    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, Self::Error> {
        let mut builder = self
            .client
            .request(request.method, request.url)
            .headers(request.headers);

        if !request.body.is_empty() {
            builder = builder.body(request.body);
        }

        let response = builder.send().await?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await?;

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

/// A blocking transport backed by [`reqwest::blocking::Client`].
#[cfg(feature = "blocking")]
#[derive(Clone, Debug, Default)]
pub struct ReqwestBlockingBackend {
    client: reqwest::blocking::Client,
}

#[cfg(feature = "blocking")]
impl ReqwestBlockingBackend {
    /// Create a backend with a default [`reqwest::blocking::Client`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a backend from an existing [`reqwest::blocking::Client`].
    pub fn with_client(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }

    /// The underlying [`reqwest::blocking::Client`].
    pub fn client(&self) -> &reqwest::blocking::Client {
        &self.client
    }
}

#[cfg(feature = "blocking")]
impl BlockingTransport for ReqwestBlockingBackend {
    type Error = reqwest::Error;

    fn execute(&self, request: HttpRequest) -> Result<HttpResponse, Self::Error> {
        let mut builder = self
            .client
            .request(request.method, request.url)
            .headers(request.headers);

        if !request.body.is_empty() {
            builder = builder.body(request.body);
        }

        let response = builder.send()?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes()?;

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}
