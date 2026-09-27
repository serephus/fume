use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;

/// The `success` field used by several Steam endpoints.
///
/// Steam famously returns `1` for success and `42` for failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ResponseResult {
    Success = 1,
    Failure = 42,
}

/// An error produced while decoding a Steam response body.
///
/// This is deliberately distinct from *transport* errors: a response can arrive
/// intact over the network yet still describe a logical failure. Keeping the two
/// apart makes error handling explicit.
#[derive(Debug, Error)]
pub enum DecodeError {
    /// The body was not valid JSON, or did not match the expected shape.
    #[error("failed to decode response: {0}")]
    Json(#[from] serde_json::Error),

    /// Steam reported a logical failure (e.g. `success: 42`).
    #[error("steam reported an error ({result:?}){}", message.as_deref().map(|m| format!(": {m}")).unwrap_or_default())]
    Steam {
        result: ResponseResult,
        message: Option<String>,
    },

    /// The response was structurally valid but semantically unexpected.
    #[error("unexpected response: {0}")]
    Unexpected(String),
}
