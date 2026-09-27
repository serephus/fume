use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{DecodeError, Endpoint, Query, decode_json};

use super::INTERFACE;

/// `ISteamWebAPIUtil/GetServerInfo/v1`
///
/// Returns the current time according to the API server.
#[derive(Clone, Debug, Default)]
pub struct GetServerInfo;

impl GetServerInfo {
    /// Create the request.
    pub fn new() -> Self {
        Self
    }
}

impl Endpoint for GetServerInfo {
    type Response = ServerInfo;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetServerInfo";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new()
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: GetServerInfoResponse = decode_json(body)?;
        Ok(ServerInfo {
            time: UNIX_EPOCH + Duration::from_secs(raw.servertime),
            time_string: raw.servertimestring,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetServerInfoResponse {
    pub servertime: u64,
    pub servertimestring: String,
}

/// Server time, with the Unix timestamp normalised to a [`SystemTime`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerInfo {
    pub time: SystemTime,
    pub time_string: String,
}
