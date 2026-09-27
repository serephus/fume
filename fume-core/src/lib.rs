//! Protocol definitions for the Steam Web API.
//!
//! `fume-core` is a pure, I/O-free crate. It describes *what* a Steam Web API
//! endpoint is — its interface, method, version, typed parameters and typed
//! response — but never performs a request. The `fume` crate turns these
//! definitions into HTTP calls, and backend crates (such as `fume-reqwest`)
//! provide the actual transport.
//!
//! This separation means the same endpoint definitions can be driven by
//! asynchronous or blocking clients alike.

mod endpoint;
mod error;
mod params;

pub mod app;
pub mod news;
pub mod player;
pub mod stats;
pub mod types;
pub mod user;
pub mod util;

pub use endpoint::Endpoint;
pub use error::{DecodeError, ResponseResult};
pub use params::{Param, Query};
pub use types::*;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// A newtype identifier.
///
/// Steam is loose about the JSON representation of identifiers: the same id
/// may arrive as a JSON number or as a quoted string depending on the endpoint.
/// This macro generates a strongly-typed wrapper that accepts either form while
/// remaining `Copy`, `Hash`, `Ord` and `Display`.
///
/// The generated type is deliberately *not* freely interchangeable with other
/// identifiers: `SteamId`, `AppId` and `GroupId` are distinct types even when
/// they share the same integer width.
#[macro_export]
macro_rules! id_type {
    ($(#[$meta:meta])* $name:ident, $inner:ty) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $name(pub $inner);

        impl $name {
            /// Construct the identifier from its raw integer value.
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            /// Return the raw integer value.
            pub const fn get(self) -> $inner {
                self.0
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, concat!(stringify!($name), "({})"), self.0)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serde::Serialize::serialize(&self.0, serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str(concat!(
                            "an integer or a numeric string for ",
                            stringify!($name)
                        ))
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name(value as $inner))
                    }

                    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name(value as $inner))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        value
                            .parse::<u64>()
                            .map(|parsed| $name(parsed as $inner))
                            .map_err(|_| {
                                E::custom(concat!(
                                    "failed to parse a numeric string as ",
                                    stringify!($name)
                                ))
                            })
                    }
                }

                deserializer.deserialize_any(Visitor)
            }
        }
    };
}

/// The generic `{ "response": ... }` envelope used by most Steam endpoints.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Response<T> {
    pub response: T,
}

/// Decode a JSON body, converting a `serde_json` failure into a [`DecodeError`].
///
/// This is the common building block for most [`Endpoint::decode`]
/// implementations.
pub fn decode_json<T: DeserializeOwned>(body: &[u8]) -> Result<T, DecodeError> {
    Ok(serde_json::from_slice(body)?)
}
