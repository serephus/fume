//! A strongly-typed Steam Web API client.
//!
//! `fume` provides a typed layer over the Steam Web API on top of the pure
//! protocol definitions in [`fume_core`]. It supports:
//!
//! * **Strongly-typed endpoints.** Every endpoint is a value with typed
//!   parameters and a typed, normalised response.
//! * **Sync and async.** A single endpoint definition is driven by either an
//!   asynchronous or a blocking transport, selected by the client's type.
//! * **Pluggable backends.** Transports are implemented in separate crates;
//!   `fume` itself has no dependency on any HTTP library.
//!
//! # Example
//!
//! ```rust,ignore
//! use fume::{ApiKey, AuthState, Client};
//! use fume_reqwest::ReqwestBackend;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let client = Client::new(ReqwestBackend::new(), ApiKey::new("KEY"));
//!
//! // Async requests implement IntoFuture and can be awaited directly.
//! let friends = client.user(76561198084913741u64).friends(None).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Blocking backends are used the same way, except the request is finished with
//! [`Request::send`] instead of `.await`.

mod auth;
mod client;
mod error;
mod handles;
mod request;
mod transport;

pub use auth::{ApiKey, AuthState, Unauthenticated};
pub use client::{Async, Blocking, Client, DEFAULT_BASE_URL, Mode};
pub use error::Error;
pub use handles::{App, User, Users};
pub use request::Request;
pub use transport::{AsyncTransport, BlockingTransport, HttpRequest, HttpResponse};

/// Re-export of the `bytes` crate used by transport types.
pub use bytes;
/// Re-export of the protocol definitions crate.
pub use fume_core;
/// Re-export of the `http` crate used by transport types.
pub use http;
