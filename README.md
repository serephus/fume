# fume

[![crates.io](https://img.shields.io/crates/v/fume)](https://crates.io/crates/fume)
[![docs.rs](https://img.shields.io/docsrs/fume)](https://docs.rs/fume)

A strongly-typed Rust client for the Steam Web API, with first-class support for
both asynchronous and blocking backends.

## Design

The workspace is split into three concerns:

| Crate | Responsibility |
| --- | --- |
| `fume-core` | Pure protocol definitions: endpoints, typed parameters, typed responses. No I/O. |
| `fume` | The client: URL building, JSON decoding, auth typestate, transport traits. No HTTP library. |
| `fume-reqwest` | A reference backend implementing both the async and blocking transport traits with `reqwest`. |

Because `fume` does not depend on any HTTP library, the same endpoint
definitions can be driven by any backend — async or blocking — without changing
application code.

## Highlights

- **Strongly-typed endpoints.** Every endpoint is a value with typed parameters
  and a normalised, typed response. `SteamId`, `AppId` and `GroupId` are
  distinct types, and responses such as timestamps are converted to `SystemTime`
  at the boundary.
- **Sync and async, one API.** Async requests implement `IntoFuture`, so they
  can be awaited directly. Blocking requests use `.send()`. The endpoint
  definitions are shared.
- **Compile-time auth.** Endpoints that need an API key are only available on
  `Client<_, ApiKey, _>`; forgetting the key is a compile error, not a 401.
- **Pluggable backends.** Transports live in their own crates. Writing a new one
  means implementing one trait.
- **Safe by default.** `ApiKey`'s `Debug` output is redacted.

## Usage

Add `fume` and a backend:

```toml
[dependencies]
fume = "0.1"
fume-reqwest = "0.1"
```

### Async

```rust,no_run
use fume::{ApiKey, Client};
use fume_reqwest::ReqwestBackend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(ReqwestBackend::new(), ApiKey::new("YOUR_KEY"));

    let friends = client.user(76561198084913741u64).friends(None).await?;
    println!("{} friends", friends.len());

    // Public endpoints work without a key.
    let client = Client::new(ReqwestBackend::new(), fume::Unauthenticated);
    let interfaces = client.apis().await?;
    println!("{} interfaces", interfaces.len());

    Ok(())
}
```

### Blocking

Enable the `blocking` feature on the backend, then call `.send()`:

```rust,no_run
use fume::{ApiKey, Client, Blocking};
use fume_reqwest::ReqwestBlockingBackend;

fn main() -> anyhow::Result<()> {
    let client: Client<_, _, Blocking> =
        Client::new(ReqwestBlockingBackend::new(), ApiKey::new("YOUR_KEY"));

    let info = client.server_info().send()?;
    println!("server time: {}", info.time_string);

    Ok(())
}
```

### Implemented endpoints

`ISteamApps` (`GetAppList`, `GetServersAtAddress`), `ISteamUser`
(`GetFriendList`, `GetPlayerSummaries`, `GetPlayerBans`, `GetUserGroupList`,
`ResolveVanityURL`), `IPlayerService` (`GetSteamLevel`, `GetOwnedGames`,
`GetRecentlyPlayedGames`, `GetBadges`), `ISteamUserStats`
(`GetNumberOfCurrentPlayers`), `ISteamNews` (`GetNewsForApp`) and
`ISteamWebAPIUtil` (`GetServerInfo`, `GetSupportedAPIList`).

Endpoints without a convenience method can be driven directly through the
endpoint types in `fume_core`:

```rust,ignore
let request = client.request(
    fume_core::news::get_news_for_app::GetNewsForApp::new(appid).count(5),
);
let news = request.await?;
```

## Writing a backend

Implement [`AsyncTransport`](https://docs.rs/fume/latest/fume/trait.AsyncTransport.html)
and/or [`BlockingTransport`](https://docs.rs/fume/latest/fume/trait.BlockingTransport.html)
for your type. A transport only has to turn an `HttpRequest` into an
`HttpResponse`; everything else is handled by `fume`.

```rust,ignore
impl AsyncTransport for MyBackend {
    type Error = MyError;

    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, Self::Error> {
        // Perform the request however you like.
    }
}
```

## Limitations

Steam's Web API is only partially documented, so response models may need to be
corrected as new data is observed. The `deny-unknown-fields` feature on
`fume-core` can help surface schema drift during development.

## License

This project is licensed under the GLWTPL (Good Luck With That Public License).
See the `LICENSE` file for more details.
