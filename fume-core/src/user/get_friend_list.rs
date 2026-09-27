use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{DecodeError, Endpoint, Query, Relationship, SteamId, decode_json};

use super::INTERFACE;

/// `ISteamUser/GetFriendList/v1`
///
/// Returns the friend list of the given user. If the user's friend list is
/// private, Steam answers with an HTTP 401.
#[derive(Clone, Debug)]
pub struct GetFriendList {
    pub steamid: SteamId,
    pub relationship: Option<Relationship>,
}

impl GetFriendList {
    /// Fetch a user's friends.
    pub fn new(steamid: impl Into<SteamId>) -> Self {
        Self {
            steamid: steamid.into(),
            relationship: None,
        }
    }

    /// Restrict the result to a given relationship type.
    #[must_use]
    pub fn relationship(mut self, relationship: Relationship) -> Self {
        self.relationship = Some(relationship);
        self
    }

    /// Restrict the result to an optional relationship type.
    #[must_use]
    pub fn relationship_opt(mut self, relationship: Option<Relationship>) -> Self {
        self.relationship = relationship;
        self
    }
}

impl Endpoint for GetFriendList {
    type Response = Vec<Friend>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetFriendList";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new()
            .param(&self.steamid)
            .param_opt(self.relationship.as_ref())
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: GetFriendListResponse = decode_json(body)?;
        Ok(raw
            .friendslist
            .friends
            .into_iter()
            .map(Friend::from)
            .collect())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetFriendListResponse {
    pub friendslist: FriendList,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FriendList {
    pub friends: Vec<RawFriend>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RawFriend {
    pub steamid: SteamId,
    pub relationship: Relationship,
    pub friend_since: u64,
}

/// A friendship, with the timestamp normalised to a [`SystemTime`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Friend {
    pub id: SteamId,
    pub relationship: Relationship,
    pub since: SystemTime,
}

impl From<RawFriend> for Friend {
    fn from(value: RawFriend) -> Self {
        Self {
            id: value.steamid,
            relationship: value.relationship,
            since: UNIX_EPOCH + Duration::from_secs(value.friend_since),
        }
    }
}
