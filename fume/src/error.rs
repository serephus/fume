use fume_core::DecodeError;
use http::StatusCode;
use thiserror::Error;

/// Everything that can go wrong while executing a request.
///
/// The transport error is a generic parameter so backends can surface their own
/// rich error types without boxing.
#[derive(Debug, Error)]
pub enum Error<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    /// The transport failed to complete the request.
    #[error("transport error: {0}")]
    Transport(#[source] E),

    /// The response body could not be interpreted.
    #[error(transparent)]
    Decode(#[from] DecodeError),

    /// The server returned a non-success HTTP status.
    #[error("unexpected HTTP status {status}: {body}")]
    Http {
        /// The status code returned.
        status: StatusCode,
        /// The (lossily decoded) response body.
        body: String,
    },

    /// The configured base URL was invalid.
    #[error("invalid base URL: {0}")]
    Url(#[from] url::ParseError),
}
