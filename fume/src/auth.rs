//! Authentication state for a [`Client`].
//!
//! The state is encoded in the type system: endpoints that require an API key
//! are only reachable on a `Client<_, ApiKey, _>`, so forgetting to attach a key
//! is a compile error rather than a runtime 401.

use std::sync::Arc;

/// The authentication information a client can supply to an endpoint.
///
/// This is a sealed, type-level marker: implement it via [`Unauthenticated`] and
/// [`ApiKey`].
pub trait AuthState: Clone + Send + Sync + 'static {
    /// The API key to attach to requests, if any.
    fn key(&self) -> Option<&str>;
}

/// A client without an API key.
///
/// Only the handful of public endpoints are reachable in this state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Unauthenticated;

impl AuthState for Unauthenticated {
    fn key(&self) -> Option<&str> {
        None
    }
}

/// A Steam Web API key.
///
/// Get one from <https://steamcommunity.com/dev/apikey>.
///
/// The key is stored behind an [`Arc`] so cloning a client is cheap, and its
/// [`Debug`](std::fmt::Debug) representation is redacted so it cannot leak into
/// logs by accident.
#[derive(Clone, PartialEq, Eq)]
pub struct ApiKey(Arc<str>);

impl ApiKey {
    /// Create a new API key.
    pub fn new(key: impl AsRef<str>) -> Self {
        Self(Arc::from(key.as_ref()))
    }

    /// Expose the raw key.
    ///
    /// Use with care: this value authenticates requests and should not be
    /// logged.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl AuthState for ApiKey {
    fn key(&self) -> Option<&str> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiKey(<redacted>)")
    }
}

impl From<&str> for ApiKey {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ApiKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for ApiKey {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}
