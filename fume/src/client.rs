use std::{marker::PhantomData, sync::Arc};

use bytes::Bytes;
use fume_core::Endpoint;
use http::{HeaderMap, HeaderValue, header};

use crate::{
    auth::{ApiKey, AuthState, Unauthenticated},
    transport::HttpRequest,
};

/// Marker for asynchronous clients.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Async {}

/// Marker for blocking clients.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blocking {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Async {}
    impl Sealed for super::Blocking {}
}

/// The execution mode of a [`Client`].
///
/// This is an implementation detail used to select the right `send`
/// implementation on a [`crate::Request`]; users never construct it.
pub trait Mode: sealed::Sealed + Clone + Copy + std::fmt::Debug + Send + Sync + 'static {}

impl Mode for Async {}
impl Mode for Blocking {}

/// The default Steam Web API host.
pub const DEFAULT_BASE_URL: &str = "https://api.steampowered.com/";

/// A Steam Web API client.
///
/// `T` is the transport, `A` the authentication state and `M` the execution
/// mode. The auth state defaults to [`Unauthenticated`] and the mode to
/// [`Async`], so the common async client is simply `Client<T>`.
///
/// A client is cheap to clone: transports and API keys are reference-counted
/// internally.
#[derive(Clone, Debug)]
pub struct Client<T, A = Unauthenticated, M = Async> {
    pub(crate) transport: T,
    pub(crate) auth: A,
    pub(crate) base_url: Arc<str>,
    pub(crate) mode: PhantomData<M>,
}

impl<T, A, M> Client<T, A, M> {
    /// Create a client from a transport and an auth state.
    pub fn new(transport: T, auth: A) -> Self {
        Self {
            transport,
            auth,
            base_url: Arc::from(DEFAULT_BASE_URL),
            mode: PhantomData,
        }
    }

    /// Override the API host (for example a Steam partner or local mirror).
    #[must_use]
    pub fn with_base_url(mut self, base_url: impl Into<Arc<str>>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// The underlying transport.
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// The current authentication state.
    pub fn auth(&self) -> &A {
        &self.auth
    }
}

impl<T, M> Client<T, Unauthenticated, M> {
    /// Attach an API key, moving the client into the authenticated state.
    pub fn with_api_key(self, key: impl Into<ApiKey>) -> Client<T, ApiKey, M> {
        Client {
            transport: self.transport,
            auth: key.into(),
            base_url: self.base_url,
            mode: PhantomData,
        }
    }
}

impl<T, A, M> Client<T, A, M>
where
    A: AuthState,
{
    pub(crate) fn build<E: Endpoint>(&self, endpoint: &E) -> Result<HttpRequest, url::ParseError> {
        let mut url = url::Url::parse(&self.base_url)?;
        let prefix = url.path().trim_end_matches('/').to_owned();
        url.set_path(&format!(
            "{prefix}/{}/{}/{}",
            E::INTERFACE,
            E::METHOD,
            E::VERSION
        ));

        {
            let mut pairs = url.query_pairs_mut();
            for (name, value) in endpoint.query().iter() {
                pairs.append_pair(name, value);
            }
            if let Some(key) = self.auth.key() {
                pairs.append_pair("key", key);
            }
        }

        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        Ok(HttpRequest {
            method: E::HTTP_METHOD,
            url: url.into(),
            headers,
            body: Bytes::new(),
        })
    }
}
