use serde::{Deserialize, Serialize};

use crate::{AppId, DecodeError, Endpoint, Query, decode_json};

use super::INTERFACE;

/// `ISteamNews/GetNewsForApp/v2`
///
/// Returns news posts for an application.
#[derive(Clone, Debug)]
pub struct GetNewsForApp {
    pub appid: AppId,
    /// Maximum number of posts to return.
    pub count: Option<u32>,
    /// Maximum length of the contents field.
    pub max_length: Option<u32>,
    /// Only return posts before this Unix timestamp.
    pub end_date: Option<u64>,
}

impl GetNewsForApp {
    /// Fetch news for `appid`.
    pub fn new(appid: impl Into<AppId>) -> Self {
        Self {
            appid: appid.into(),
            count: None,
            max_length: None,
            end_date: None,
        }
    }

    /// Limit the number of posts.
    #[must_use]
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// Truncate post bodies to `max_length` characters.
    #[must_use]
    pub fn max_length(mut self, max_length: u32) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Only return posts before `end_date`.
    #[must_use]
    pub fn end_date(mut self, end_date: u64) -> Self {
        self.end_date = Some(end_date);
        self
    }
}

impl Endpoint for GetNewsForApp {
    type Response = Vec<NewsItem>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetNewsForApp";
    const VERSION: &'static str = "v2";

    fn query(&self) -> Query {
        Query::new()
            .param(&self.appid)
            .push_opt("count", self.count)
            .push_opt("maxlength", self.max_length)
            .push_opt("enddate", self.end_date)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: GetNewsForAppResponse = decode_json(body)?;
        Ok(raw.appnews.newsitems)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNewsForAppResponse {
    pub appnews: AppNews,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AppNews {
    pub appid: AppId,
    #[serde(default)]
    pub newsitems: Vec<NewsItem>,
}

/// A single news post.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct NewsItem {
    pub gid: String,
    pub title: String,
    pub url: String,
    pub is_external_url: bool,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub contents: String,
    #[serde(default)]
    pub feedlabel: String,
    pub date: u64,
    #[serde(default)]
    pub feedname: String,
    #[serde(default)]
    pub feed_type: i32,
    pub appid: AppId,
}
