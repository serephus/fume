use std::{future::Future, pin::Pin};

use fume_core::Endpoint;

use crate::{
    auth::AuthState,
    client::{Async, Blocking, Client},
    error::Error,
    transport::{AsyncTransport, BlockingTransport, HttpResponse},
};

/// A built request: an endpoint plus the client that will execute it.
///
/// A request performs no I/O until it is sent. This makes it a natural place to
/// add logging, retries or caching later, and lets the same endpoint be driven
/// by either an asynchronous or a blocking client.
///
/// # Async
///
/// Asynchronous requests implement [`IntoFuture`](std::future::IntoFuture), so
/// they can be awaited directly:
///
/// ```rust,ignore
/// let friends = client.user(steamid).friends(None).await?;
/// ```
///
/// # Blocking
///
/// Blocking requests are executed with [`send`](Request::send):
///
/// ```rust,ignore
/// let friends = client.user(steamid).friends(None).send()?;
/// ```
#[derive(Clone, Debug)]
pub struct Request<E, T, A, M> {
    pub(crate) endpoint: E,
    pub(crate) client: Client<T, A, M>,
}

impl<E, T, A, M> Request<E, T, A, M> {
    pub(crate) fn new(endpoint: E, client: Client<T, A, M>) -> Self {
        Self { endpoint, client }
    }

    /// The endpoint being requested.
    pub fn endpoint(&self) -> &E {
        &self.endpoint
    }

    /// Consume the request and return the endpoint.
    pub fn into_endpoint(self) -> E {
        self.endpoint
    }

    /// The client that will execute the request.
    pub fn client(&self) -> &Client<T, A, M> {
        &self.client
    }
}

impl<E, T, A> Request<E, T, A, Async>
where
    E: Endpoint,
    T: AsyncTransport,
    A: AuthState,
{
    /// Execute the request asynchronously.
    pub async fn send(self) -> Result<E::Response, Error<T::Error>> {
        let request = self.client.build(&self.endpoint)?;
        let response = self
            .client
            .transport
            .execute(request)
            .await
            .map_err(Error::Transport)?;
        decode::<E, T::Error>(response)
    }
}

impl<E, T, A> Request<E, T, A, Blocking>
where
    E: Endpoint,
    T: BlockingTransport,
    A: AuthState,
{
    /// Execute the request on the current thread.
    pub fn send(self) -> Result<E::Response, Error<T::Error>> {
        let request = self.client.build(&self.endpoint)?;
        let response = self
            .client
            .transport
            .execute(request)
            .map_err(Error::Transport)?;
        decode::<E, T::Error>(response)
    }
}

impl<E, T, A> std::future::IntoFuture for Request<E, T, A, Async>
where
    E: Endpoint + Send + 'static,
    E::Response: 'static,
    T: AsyncTransport,
    A: AuthState,
{
    type Output = Result<E::Response, Error<T::Error>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

fn decode<E, Err>(response: HttpResponse) -> Result<E::Response, Error<Err>>
where
    E: Endpoint,
    Err: std::error::Error + Send + Sync + 'static,
{
    if !response.status.is_success() {
        return Err(Error::Http {
            status: response.status,
            body: String::from_utf8_lossy(&response.body).into_owned(),
        });
    }
    Ok(E::decode(&response.body)?)
}
