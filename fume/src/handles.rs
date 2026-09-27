//! Ergonomic, typed handles over the raw endpoint definitions.

use fume_core::{
    AppId, Relationship, SteamId, SteamIds,
    app::{get_app_list::GetAppList, get_servers_at_address::GetServersAtAddress},
    news::get_news_for_app::GetNewsForApp,
    player::{
        get_badges::GetBadges, get_owned_games::GetOwnedGames,
        get_recently_played_games::GetRecentlyPlayedGames, get_steam_level::GetSteamLevel,
    },
    stats::get_number_of_current_players::GetNumberOfCurrentPlayers,
    user::{
        get_friend_list::GetFriendList, get_player_bans::GetPlayerBans,
        get_player_summaries::GetPlayerSummaries, get_user_group_list::GetUserGroupList,
        resolve_vanity_url::ResolveVanityUrl,
    },
    util::{get_server_info::GetServerInfo, get_supported_api_list::GetSupportedApiList},
};

use crate::{
    auth::{ApiKey, AuthState},
    client::{Client, Mode},
    request::Request,
};

impl<T, A, M> Client<T, A, M>
where
    T: Clone,
    A: Clone,
    M: Mode,
{
    /// Wrap an arbitrary endpoint in a request without going through a handle.
    ///
    /// This is the escape hatch for endpoints that do not (yet) have a
    /// convenience method.
    pub fn request<E>(&self, endpoint: E) -> Request<E, T, A, M> {
        Request::new(endpoint, self.clone())
    }
}

impl<T, A, M> Client<T, A, M>
where
    T: Clone,
    A: AuthState,
    M: Mode,
{
    /// A handle for a single application.
    pub fn app(&self, appid: impl Into<AppId>) -> App<Client<T, A, M>> {
        App {
            client: self.clone(),
            id: appid.into(),
        }
    }

    /// `ISteamWebAPIUtil/GetSupportedAPIList` — every interface and method.
    pub fn apis(&self) -> Request<GetSupportedApiList, T, A, M> {
        self.request(GetSupportedApiList::new())
    }

    /// `ISteamWebAPIUtil/GetServerInfo` — the API server's current time.
    pub fn server_info(&self) -> Request<GetServerInfo, T, A, M> {
        self.request(GetServerInfo::new())
    }

    /// `ISteamApps/GetAppList` — the public application catalogue.
    pub fn apps(&self) -> Request<GetAppList, T, A, M> {
        self.request(GetAppList::new())
    }

    /// `ISteamApps/GetServersAtAddress` — game servers at an address.
    pub fn servers_at_address(
        &self,
        addr: impl Into<String>,
    ) -> Request<GetServersAtAddress, T, A, M> {
        self.request(GetServersAtAddress::new(addr))
    }
}

impl<T, M> Client<T, ApiKey, M>
where
    T: Clone,
    M: Mode,
{
    /// A handle for a single user.
    pub fn user(&self, steamid: impl Into<SteamId>) -> User<Client<T, ApiKey, M>> {
        User {
            client: self.clone(),
            id: steamid.into(),
        }
    }

    /// A handle for a batch of users.
    pub fn users<I>(&self, steamids: I) -> Users<Client<T, ApiKey, M>>
    where
        I: IntoIterator,
        I::Item: Into<SteamId>,
    {
        Users {
            client: self.clone(),
            ids: steamids.into_iter().map(Into::into).collect(),
        }
    }

    /// `ISteamUser/ResolveVanityURL` — resolve a custom URL to a SteamID.
    pub fn resolve_vanity_url(
        &self,
        vanity_url: impl Into<String>,
    ) -> Request<ResolveVanityUrl, T, ApiKey, M> {
        self.request(ResolveVanityUrl::new(vanity_url))
    }
}

/// A single Steam user.
#[derive(Clone, Debug)]
pub struct User<C> {
    pub(crate) client: C,
    pub(crate) id: SteamId,
}

impl<C> User<C> {
    /// This user's SteamID.
    pub fn id(&self) -> SteamId {
        self.id
    }

    /// Consume the handle and return the SteamID.
    pub fn into_id(self) -> SteamId {
        self.id
    }
}

impl<T, M> User<Client<T, ApiKey, M>>
where
    T: Clone,
    M: Mode,
{
    /// `ISteamUser/GetFriendList`
    pub fn friends(
        &self,
        relationship: Option<Relationship>,
    ) -> Request<GetFriendList, T, ApiKey, M> {
        self.client
            .request(GetFriendList::new(self.id).relationship_opt(relationship))
    }

    /// `ISteamUser/GetUserGroupList`
    pub fn groups(&self) -> Request<GetUserGroupList, T, ApiKey, M> {
        self.client.request(GetUserGroupList::new(self.id))
    }

    /// `ISteamUser/GetPlayerSummaries` for this single user.
    pub fn summary(&self) -> Request<GetPlayerSummaries, T, ApiKey, M> {
        self.client
            .request(GetPlayerSummaries::new(SteamIds(vec![self.id])))
    }

    /// `ISteamUser/GetPlayerBans` for this single user.
    pub fn bans(&self) -> Request<GetPlayerBans, T, ApiKey, M> {
        self.client
            .request(GetPlayerBans::new(SteamIds(vec![self.id])))
    }

    /// `IPlayerService/GetSteamLevel`
    pub fn level(&self) -> Request<GetSteamLevel, T, ApiKey, M> {
        self.client.request(GetSteamLevel::new(self.id))
    }

    /// `IPlayerService/GetOwnedGames`, including app info and played free games.
    pub fn owned_games(&self) -> Request<GetOwnedGames, T, ApiKey, M> {
        self.client.request(
            GetOwnedGames::new(self.id)
                .include_app_info(true)
                .include_played_free_games(true),
        )
    }

    /// `IPlayerService/GetRecentlyPlayedGames`
    pub fn recently_played_games(&self) -> Request<GetRecentlyPlayedGames, T, ApiKey, M> {
        self.client.request(GetRecentlyPlayedGames::new(self.id))
    }

    /// `IPlayerService/GetBadges`
    pub fn badges(&self) -> Request<GetBadges, T, ApiKey, M> {
        self.client.request(GetBadges::new(self.id))
    }
}

/// A batch of Steam users, for endpoints that accept many ids at once.
#[derive(Clone, Debug)]
pub struct Users<C> {
    pub(crate) client: C,
    pub(crate) ids: Vec<SteamId>,
}

impl<C> Users<C> {
    /// The SteamIDs in this batch.
    pub fn ids(&self) -> &[SteamId] {
        &self.ids
    }
}

impl<T, M> Users<Client<T, ApiKey, M>>
where
    T: Clone,
    M: Mode,
{
    /// `ISteamUser/GetPlayerSummaries`
    pub fn summaries(&self) -> Request<GetPlayerSummaries, T, ApiKey, M> {
        self.client
            .request(GetPlayerSummaries::new(SteamIds(self.ids.clone())))
    }

    /// `ISteamUser/GetPlayerBans`
    pub fn bans(&self) -> Request<GetPlayerBans, T, ApiKey, M> {
        self.client
            .request(GetPlayerBans::new(SteamIds(self.ids.clone())))
    }
}

/// A single application.
#[derive(Clone, Debug)]
pub struct App<C> {
    pub(crate) client: C,
    pub(crate) id: AppId,
}

impl<C> App<C> {
    /// This app's id.
    pub fn id(&self) -> AppId {
        self.id
    }
}

impl<T, A, M> App<Client<T, A, M>>
where
    T: Clone,
    A: AuthState,
    M: Mode,
{
    /// `ISteamUserStats/GetNumberOfCurrentPlayers`
    pub fn current_players(&self) -> Request<GetNumberOfCurrentPlayers, T, A, M> {
        self.client.request(GetNumberOfCurrentPlayers::new(self.id))
    }

    /// `ISteamNews/GetNewsForApp`
    pub fn news(&self) -> Request<GetNewsForApp, T, A, M> {
        self.client.request(GetNewsForApp::new(self.id))
    }
}
