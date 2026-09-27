use serde::{Deserialize, Serialize};

use crate::{
    DecodeError, Endpoint, Query, Response, SteamId, decode_json,
    player::get_owned_games::OwnedGame,
};

use super::INTERFACE;

/// `IPlayerService/GetRecentlyPlayedGames/v1`
///
/// Returns the games a player has played in the last two weeks.
#[derive(Clone, Debug)]
pub struct GetRecentlyPlayedGames {
    pub steamid: SteamId,
    /// Maximum number of games to return.
    pub count: Option<u32>,
}

impl GetRecentlyPlayedGames {
    /// Fetch recently played games for `steamid`.
    pub fn new(steamid: impl Into<SteamId>) -> Self {
        Self {
            steamid: steamid.into(),
            count: None,
        }
    }

    /// Limit the number of returned games.
    #[must_use]
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
}

impl Endpoint for GetRecentlyPlayedGames {
    type Response = RecentlyPlayedGames;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetRecentlyPlayedGames";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new()
            .param(&self.steamid)
            .push_opt("count", self.count)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<RecentlyPlayedGames> = decode_json(body)?;
        Ok(raw.response)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RecentlyPlayedGames {
    #[serde(default)]
    pub total_count: u32,
    #[serde(default)]
    pub games: Vec<OwnedGame>,
}
