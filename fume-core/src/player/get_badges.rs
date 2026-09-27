use serde::{Deserialize, Serialize};

use crate::{AppId, DecodeError, Endpoint, Query, Response, SteamId, decode_json};

use super::INTERFACE;

/// `IPlayerService/GetBadges/v1`
///
/// Returns a player's badges and XP.
#[derive(Clone, Debug)]
pub struct GetBadges {
    pub steamid: SteamId,
}

impl GetBadges {
    /// Fetch badges for `steamid`.
    pub fn new(steamid: impl Into<SteamId>) -> Self {
        Self {
            steamid: steamid.into(),
        }
    }
}

impl Endpoint for GetBadges {
    type Response = PlayerBadges;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetBadges";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new().param(&self.steamid)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<PlayerBadges> = decode_json(body)?;
        Ok(raw.response)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PlayerBadges {
    #[serde(default)]
    pub player_xp: u32,
    #[serde(default)]
    pub player_level: u32,
    #[serde(default)]
    pub player_xp_needed_to_level_up: u32,
    #[serde(default)]
    pub player_xp_needed_current_level: u32,
    #[serde(default)]
    pub badges: Vec<Badge>,
}

/// A single badge owned by a player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Badge {
    pub badgeid: u32,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub completion_time: u64,
    #[serde(default)]
    pub xp: u32,
    #[serde(default)]
    pub scarcity: u32,
    #[serde(default)]
    pub appid: Option<AppId>,
    #[serde(default)]
    pub communityitemid: Option<String>,
    #[serde(default)]
    pub border_color: Option<u32>,
}
