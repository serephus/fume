use serde::{Deserialize, Serialize};

use crate::{DecodeError, Endpoint, GroupId, Query, Response, SteamId, decode_json};

use super::INTERFACE;

/// `ISteamUser/GetUserGroupList/v1`
///
/// Returns the groups (clans) a user belongs to.
#[derive(Clone, Debug)]
pub struct GetUserGroupList {
    pub steamid: SteamId,
}

impl GetUserGroupList {
    /// Fetch the groups for `steamid`.
    pub fn new(steamid: impl Into<SteamId>) -> Self {
        Self {
            steamid: steamid.into(),
        }
    }
}

impl Endpoint for GetUserGroupList {
    type Response = Vec<GroupId>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetUserGroupList";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new().param(&self.steamid)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<GetUserGroupListInner> = decode_json(body)?;
        Ok(raw
            .response
            .groups
            .into_iter()
            .map(|group| group.gid)
            .collect())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetUserGroupListInner {
    pub success: bool,
    #[serde(default)]
    pub groups: Vec<UserGroup>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UserGroup {
    pub gid: GroupId,
}
