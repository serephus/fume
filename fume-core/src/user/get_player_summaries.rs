use serde::{Deserialize, Serialize};

use crate::{
    AppId, CommentPermission, CommunityVisibilityState, DecodeError, Endpoint, GroupId,
    PersonaState, ProfileState, Query, Response, SteamId, SteamIds, decode_json,
};

use super::INTERFACE;

/// `ISteamUser/GetPlayerSummaries/v2`
///
/// Fetches profile summaries for up to 100 SteamIDs at once.
#[derive(Clone, Debug)]
pub struct GetPlayerSummaries {
    pub steamids: SteamIds,
}

impl GetPlayerSummaries {
    /// Fetch summaries for the given SteamIDs.
    pub fn new(steamids: impl Into<SteamIds>) -> Self {
        Self {
            steamids: steamids.into(),
        }
    }
}

impl Endpoint for GetPlayerSummaries {
    type Response = Vec<PlayerSummary>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetPlayerSummaries";
    const VERSION: &'static str = "v2";

    fn query(&self) -> Query {
        Query::new().param(&self.steamids)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<PlayerSummaries> = decode_json(body)?;
        Ok(raw.response.players)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PlayerSummaries {
    pub players: Vec<PlayerSummary>,
}

/// A player's public profile summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PlayerSummary {
    pub steamid: SteamId,
    #[serde(rename = "communityvisibilitystate")]
    pub community_visibility_state: CommunityVisibilityState,
    #[serde(rename = "profilestate")]
    pub profile_state: Option<ProfileState>,
    #[serde(rename = "personaname")]
    pub persona_name: String,
    #[serde(rename = "profileurl")]
    pub profile_url: String,
    pub avatar: String,
    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,
    #[serde(rename = "avatarfull")]
    pub avatar_full: String,
    #[serde(rename = "avatarhash")]
    pub avatar_hash: String,
    #[serde(rename = "lastlogoff")]
    pub last_logoff: Option<u64>,
    #[serde(rename = "personastate")]
    pub persona_state: PersonaState,
    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: Option<GroupId>,
    #[serde(rename = "timecreated")]
    pub time_created: Option<u64>,
    #[serde(rename = "personastateflags")]
    pub persona_state_flags: Option<u8>,
    #[serde(rename = "gameid")]
    pub game_id: Option<AppId>,
    #[serde(rename = "gameserverip")]
    pub game_server_ip: Option<String>,
    #[serde(rename = "gameextrainfo")]
    pub game_extra_info: Option<String>,
    #[serde(rename = "commentpermission")]
    pub comment_permission: Option<CommentPermission>,
    #[serde(rename = "realname")]
    pub real_name: Option<String>,
    #[serde(rename = "loccityid")]
    pub loc_city_id: Option<u64>,
    #[serde(rename = "loccountrycode")]
    pub loc_country_code: Option<String>,
    #[serde(rename = "locstatecode")]
    pub loc_state_code: Option<String>,
}
