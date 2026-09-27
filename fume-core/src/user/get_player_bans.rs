use serde::{Deserialize, Serialize};

use crate::{DecodeError, EconomyBan, Endpoint, Query, SteamId, SteamIds, decode_json};

use super::INTERFACE;

/// `ISteamUser/GetPlayerBans/v1`
///
/// Returns ban information for one or more players.
#[derive(Clone, Debug)]
pub struct GetPlayerBans {
    pub steamids: SteamIds,
}

impl GetPlayerBans {
    /// Fetch bans for the given SteamIDs.
    pub fn new(steamids: impl Into<SteamIds>) -> Self {
        Self {
            steamids: steamids.into(),
        }
    }
}

impl Endpoint for GetPlayerBans {
    type Response = Vec<PlayerBan>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetPlayerBans";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new().param(&self.steamids)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: GetPlayerBansResponse = decode_json(body)?;
        Ok(raw.players)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPlayerBansResponse {
    pub players: Vec<PlayerBan>,
}

/// Ban status for a single player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PlayerBan {
    #[serde(rename = "SteamId")]
    pub steamid: SteamId,
    #[serde(rename = "CommunityBanned")]
    pub community_banned: bool,
    #[serde(rename = "VACBanned")]
    pub vac_banned: bool,
    #[serde(rename = "NumberOfVACBans")]
    pub number_of_vac_bans: u32,
    #[serde(rename = "DaysSinceLastBan")]
    pub days_since_last_ban: u32,
    #[serde(rename = "NumberOfGameBans")]
    pub number_of_game_bans: u32,
    #[serde(rename = "EconomyBan")]
    pub economy_ban: EconomyBan,
}
