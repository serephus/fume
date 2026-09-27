//! `ISteamUser` endpoints.

pub(crate) const INTERFACE: &str = "ISteamUser";

pub mod get_friend_list;
pub mod get_player_bans;
pub mod get_player_summaries;
pub mod get_user_group_list;
pub mod resolve_vanity_url;
