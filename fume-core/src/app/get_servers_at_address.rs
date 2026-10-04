use serde::{Deserialize, Serialize};

use crate::{AppId, DecodeError, Endpoint, Query, Response, SteamId, decode_json};

use super::INTERFACE;

/// `ISteamApps/GetServersAtAddress/v1`
///
/// Lists the game servers currently registered at a given address.
#[derive(Clone, Debug)]
pub struct GetServersAtAddress {
    /// IPv4 address or `host:port` to query.
    pub addr: String,
}

impl GetServersAtAddress {
    /// Query the servers at `addr`.
    pub fn new(addr: impl Into<String>) -> Self {
        Self { addr: addr.into() }
    }
}

impl Endpoint for GetServersAtAddress {
    type Response = Vec<GameServer>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetServersAtAddress";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new().push("addr", &self.addr)
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: Response<GetServersAtAddressInner> = decode_json(body)?;
        Ok(raw.response.servers)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetServersAtAddressInner {
    pub success: bool,
    #[serde(default)]
    pub servers: Vec<GameServer>,
}

/// A game server returned by [`GetServersAtAddress`].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GameServer {
    pub addr: String,
    pub gameport: u16,
    pub steamid: SteamId,
    pub name: String,
    pub appid: AppId,
    pub gamedir: String,
    pub version: String,
    pub product: String,
    pub region: i32,
    pub players: u32,
    pub max_players: u32,
    pub bots: u32,
    pub map: String,
    pub secure: bool,
    pub dedicated: bool,
    pub os: String,
    #[serde(default)]
    pub gametype: String,
}
