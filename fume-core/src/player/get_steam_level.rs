use serde::{Deserialize, Serialize};

use crate::{DecodeError, Endpoint, Query, Response, SteamId, decode_json};

use super::INTERFACE;

/// `IPlayerService/GetSteamLevel/v1`
///
/// Returns a player's Steam level.
#[derive(Clone, Debug)]
pub struct GetSteamLevel {
    pub steamid: SteamId,
}

impl GetSteamLevel {
    /// Fetch the Steam level for `steamid`.
    pub fn new(steamid: impl Into<SteamId>) -> Self {
        Self {
            steamid: steamid.into(),
        }
    }
}

impl Endpoint for GetSteamLevel {
    type Response = u32;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetSteamLevel";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new().param(&self.steamid)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<SteamLevel> = decode_json(body)?;
        Ok(raw.response.player_level)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SteamLevel {
    pub player_level: u32,
}
