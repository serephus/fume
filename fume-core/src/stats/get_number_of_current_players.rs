use serde::{Deserialize, Serialize};

use crate::{AppId, DecodeError, Endpoint, Query, Response, decode_json};

use super::INTERFACE;

/// `ISteamUserStats/GetNumberOfCurrentPlayers/v1`
///
/// Returns how many players are currently in-game for an app.
#[derive(Clone, Debug)]
pub struct GetNumberOfCurrentPlayers {
    pub appid: AppId,
}

impl GetNumberOfCurrentPlayers {
    /// Query the current player count for `appid`.
    pub fn new(appid: impl Into<AppId>) -> Self {
        Self {
            appid: appid.into(),
        }
    }
}

impl Endpoint for GetNumberOfCurrentPlayers {
    type Response = u32;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetNumberOfCurrentPlayers";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new().param(&self.appid)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<CurrentPlayers> = decode_json(body)?;
        Ok(raw.response.player_count)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CurrentPlayers {
    pub player_count: u32,
    #[serde(default)]
    pub result: u32,
}
