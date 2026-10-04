use serde::{Deserialize, Serialize};

use crate::{AppId, DecodeError, Endpoint, Query, Response, SteamId, decode_json};

use super::INTERFACE;

/// `IPlayerService/GetOwnedGames/v1`
///
/// Returns the games a player owns, with playtime information.
#[derive(Clone, Debug)]
pub struct GetOwnedGames {
    pub steamid: SteamId,
    /// Include the game name and icon in the response.
    pub include_app_info: Option<bool>,
    /// Include free-to-play games that have been played.
    pub include_played_free_games: Option<bool>,
    /// Include free games (subscriptions).
    pub include_free_sub: Option<bool>,
    /// Skip games that have not been through Steam's vetting process.
    pub skip_unvetted_apps: Option<bool>,
    /// Include additional metadata such as content descriptors.
    pub include_extended_app_info: Option<bool>,
    /// Localisation language for names.
    pub language: Option<String>,
}

impl GetOwnedGames {
    /// Fetch owned games for `steamid`.
    pub fn new(steamid: impl Into<SteamId>) -> Self {
        Self {
            steamid: steamid.into(),
            include_app_info: None,
            include_played_free_games: None,
            include_free_sub: None,
            skip_unvetted_apps: None,
            include_extended_app_info: None,
            language: None,
        }
    }

    /// Include the name and icon for each game.
    #[must_use]
    pub fn include_app_info(mut self, value: bool) -> Self {
        self.include_app_info = Some(value);
        self
    }

    /// Include free-to-play games that have been played.
    #[must_use]
    pub fn include_played_free_games(mut self, value: bool) -> Self {
        self.include_played_free_games = Some(value);
        self
    }

    /// Set the localisation language.
    #[must_use]
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

impl Endpoint for GetOwnedGames {
    type Response = OwnedGames;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetOwnedGames";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new()
            .param(&self.steamid)
            .push_opt("include_appinfo", self.include_app_info)
            .push_opt("include_played_free_games", self.include_played_free_games)
            .push_opt("include_free_sub", self.include_free_sub)
            .push_opt("skip_unvetted_apps", self.skip_unvetted_apps)
            .push_opt("include_extended_appinfo", self.include_extended_app_info)
            .push_opt("language", self.language.as_deref())
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<OwnedGames> = decode_json(body)?;
        Ok(raw.response)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct OwnedGames {
    #[serde(default)]
    pub game_count: u32,
    #[serde(default)]
    pub games: Vec<OwnedGame>,
}

/// A single owned game.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct OwnedGame {
    pub appid: AppId,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub playtime_forever: u32,
    #[serde(default)]
    pub playtime_2weeks: Option<u32>,
    #[serde(default)]
    pub playtime_windows_forever: Option<u32>,
    #[serde(default)]
    pub playtime_mac_forever: Option<u32>,
    #[serde(default)]
    pub playtime_linux_forever: Option<u32>,
    #[serde(default)]
    pub playtime_deck_forever: Option<u32>,
    #[serde(default)]
    pub rtime_last_played: Option<u64>,
    #[serde(default)]
    pub img_icon_url: Option<String>,
    #[serde(default)]
    pub has_community_visible_stats: Option<bool>,
    #[serde(default)]
    pub content_descriptorids: Option<Vec<AppId>>,
}
