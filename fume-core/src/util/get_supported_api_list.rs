use serde::{Deserialize, Serialize};

use crate::{DecodeError, Endpoint, Query, decode_json};

use super::INTERFACE;

/// `ISteamWebAPIUtil/GetSupportedAPIList/v1`
///
/// Lists the interfaces and methods the API exposes. The response differs
/// depending on whether the request carries an API key.
#[derive(Clone, Debug, Default)]
pub struct GetSupportedApiList;

impl GetSupportedApiList {
    /// Create the request.
    pub fn new() -> Self {
        Self
    }
}

impl Endpoint for GetSupportedApiList {
    type Response = Vec<Interface>;

    const INTERFACE: &'static str = INTERFACE;
    const METHOD: &'static str = "GetSupportedAPIList";
    const VERSION: &'static str = "v1";

    fn query(&self) -> Query {
        Query::new()
    }

    fn decode(body: &[u8]) -> Result<Self::Response, DecodeError> {
        let raw: GetSupportedApiListResponse = decode_json(body)?;
        Ok(raw.apilist.interfaces)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Bool,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    String,
    #[serde(rename = "{enum}")]
    Enum,
    #[serde(rename = "{message}")]
    Message,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Parameter {
    pub name: String,
    pub r#type: ParameterType,
    pub optional: bool,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Method {
    pub name: String,
    pub version: i32,
    pub httpmethod: HttpMethod,
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Interface {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ApiList {
    pub interfaces: Vec<Interface>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetSupportedApiListResponse {
    pub apilist: ApiList,
}
