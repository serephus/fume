use http::Method;

use crate::{DecodeError, Query};

/// A single Steam Web API endpoint.
///
/// An endpoint is described by its `{interface}/{method}/{version}` path, the
/// HTTP method used to reach it, the typed query it accepts, and how to turn a
/// response body into a strongly-typed value.
///
/// Endpoints are plain values. Building one performs no I/O, which makes them
/// easy to inspect, log, retry or serialize. The `fume` crate wraps an endpoint
/// together with a client into a request that can be executed either
/// asynchronously or on a blocking thread.
pub trait Endpoint {
    /// The strongly-typed value produced by a successful call.
    ///
    /// This is the *clean* representation: raw JSON shapes (timestamp integers,
    /// nested envelopes, error flags) are normalised here.
    type Response;

    /// Steam interface name, e.g. `ISteamUser`.
    const INTERFACE: &'static str;
    /// Steam method name, e.g. `GetFriendList`.
    const METHOD: &'static str;
    /// Steam method version, e.g. `v1`.
    const VERSION: &'static str;
    /// HTTP method. Defaults to `GET`; Steam almost exclusively uses `GET`.
    const HTTP_METHOD: Method = Method::GET;

    /// The query parameters for this endpoint (excluding the API key, which is
    /// injected by the client).
    fn query(&self) -> Query;

    /// Decode a successful response body into [`Endpoint::Response`].
    ///
    /// Implementations are encouraged to use [`crate::decode_json`].
    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError>;
}
