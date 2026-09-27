use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::Param;

crate::id_type!(SteamId, u64);
crate::id_type!(AppId, u32);
crate::id_type!(GroupId, u64);
crate::id_type!(PublishedFileId, u64);

impl SteamId {
    /// The offset between a 64-bit SteamID and a 32-bit account id.
    pub const BASE: u64 = 76_561_197_960_265_728;

    /// The 32-bit account id (`STEAM_0:...` form) encoded by this SteamID.
    pub const fn account_id(self) -> u32 {
        self.0.wrapping_sub(Self::BASE) as u32
    }

    /// Build a SteamID from a 32-bit account id.
    pub const fn from_account_id(id: u32) -> Self {
        Self(Self::BASE + id as u64)
    }
}

impl From<u32> for SteamId {
    fn from(value: u32) -> Self {
        Self::from_account_id(value)
    }
}

impl From<SteamId> for u32 {
    fn from(value: SteamId) -> Self {
        value.account_id()
    }
}

impl Param for SteamId {
    const NAME: &'static str = "steamid";

    fn value(&self) -> String {
        self.0.to_string()
    }
}

impl Param for AppId {
    const NAME: &'static str = "appid";

    fn value(&self) -> String {
        self.0.to_string()
    }
}

impl Param for GroupId {
    const NAME: &'static str = "gid";

    fn value(&self) -> String {
        self.0.to_string()
    }
}

impl Param for PublishedFileId {
    const NAME: &'static str = "publishedfileid";

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// A batch of SteamIDs, rendered as a single comma-separated query parameter.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SteamIds(pub Vec<SteamId>);

impl SteamIds {
    /// Create a batch from anything iterable over SteamIDs.
    pub fn new(ids: impl IntoIterator<Item = SteamId>) -> Self {
        Self(ids.into_iter().collect())
    }
}

impl From<Vec<SteamId>> for SteamIds {
    fn from(value: Vec<SteamId>) -> Self {
        Self(value)
    }
}

impl Param for SteamIds {
    const NAME: &'static str = "steamids";

    fn value(&self) -> String {
        self.0
            .iter()
            .map(|id| id.0.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// The direction of a friendship relationship.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Relationship {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "friend")]
    Friend,
}

impl Param for Relationship {
    const NAME: &'static str = "relationship";

    fn value(&self) -> String {
        match self {
            Self::All => "all".to_owned(),
            Self::Friend => "friend".to_owned(),
        }
    }
}

/// The kind of vanity URL being resolved.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UrlType {
    #[default]
    IndividualProfile,
    Group,
    OfficialGameGroup,
}

impl Param for UrlType {
    const NAME: &'static str = "url_type";

    fn value(&self) -> String {
        match self {
            Self::IndividualProfile => "1",
            Self::Group => "2",
            Self::OfficialGameGroup => "3",
        }
        .to_owned()
    }
}

/// A user's current persona state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PersonaState {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
    Snooze = 4,
    LookingToTrade = 5,
    LookingToPlay = 6,
}

/// Who can view a profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CommunityVisibilityState {
    Private = 1,
    FriendsOnly = 2,
    Public = 3,
}

/// Whether a profile has been configured.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ProfileState {
    Unconfigured = 0,
    Configured = 1,
}

/// Whether comments are permitted on a profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CommentPermission {
    Disabled = 0,
    Enabled = 1,
}

/// The economy-ban status returned by `GetPlayerBans`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EconomyBan {
    None,
    Probation,
    Banned,
}
