use serde::{Deserialize, Serialize};

use crate::{AppId, DecodeError, Endpoint, Query, decode_json};

use super::INTERFACE;

/// `ISteamApps/GetAppList/v2`
///
/// Returns the full list of publicly visible applications. The list is large
/// (tens of thousands of entries); use the optional filters to page through it.
#[derive(Clone, Debug, Default)]
pub struct GetAppList {
    /// Only return apps changed since this Unix timestamp.
    pub if_modified_since: Option<u32>,
    /// Return apps with an id greater than this.
    pub last_appid: Option<AppId>,
    /// Maximum number of apps to return.
    pub max_results: Option<u32>,
}

impl GetAppList {
    /// An unfiltered listing.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Endpoint for GetAppList {
    type Response = Vec<App>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetAppList";
    const VERSION: &'static str = "v2";

    fn query(&self) -> Query {
        Query::new()
            .push_opt("if_modified_since", self.if_modified_since)
            .push_opt("last_appid", self.last_appid)
            .push_opt("max_results", self.max_results)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: GetAppListResponse = decode_json(body)?;
        Ok(raw.applist.apps)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAppListResponse {
    pub applist: AppList,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AppList {
    pub apps: Vec<App>,
}

/// A single Steam application.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct App {
    pub appid: AppId,
    pub name: String,
}
