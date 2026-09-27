use serde::{Deserialize, Serialize};

use crate::{
    DecodeError, Endpoint, Query, Response, ResponseResult, SteamId, UrlType, decode_json,
};

use super::INTERFACE;

/// `ISteamUser/ResolveVanityURL/v1`
///
/// Resolves a custom profile URL (`https://steamcommunity.com/id/<vanity>`) to a
/// 64-bit SteamID. A `success: 42` response means the vanity URL did not match
/// anything and is reported as `None` rather than an error.
#[derive(Clone, Debug)]
pub struct ResolveVanityUrl {
    pub vanity_url: String,
    pub url_type: Option<UrlType>,
}

impl ResolveVanityUrl {
    /// Resolve `vanity_url` as an individual profile.
    pub fn new(vanity_url: impl Into<String>) -> Self {
        Self {
            vanity_url: vanity_url.into(),
            url_type: None,
        }
    }

    /// Resolve a specific kind of vanity URL.
    #[must_use]
    pub fn url_type(mut self, url_type: UrlType) -> Self {
        self.url_type = Some(url_type);
        self
    }
}

impl Endpoint for ResolveVanityUrl {
    type Response = Option<SteamId>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "ResolveVanityURL";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new()
            .push("vanityurl", &self.vanity_url)
            .param_opt(self.url_type.as_ref())
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<ResolveVanityUrlInner> = decode_json(body)?;
        match raw.response.success {
            ResponseResult::Success => match raw.response.steamid {
                Some(steamid) => Ok(Some(steamid)),
                None => Err(DecodeError::Unexpected(
                    "ResolveVanityURL reported success without a steamid".to_owned(),
                )),
            },
            ResponseResult::Failure => Ok(None),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ResolveVanityUrlInner {
    pub success: ResponseResult,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub steamid: Option<SteamId>,
}
