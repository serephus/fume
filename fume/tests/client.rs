//! Tests for the client layer using an in-memory transport.

use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use fume::{
    ApiKey, AsyncTransport, Blocking, BlockingTransport, Client, HttpRequest, HttpResponse,
    Unauthenticated, http::StatusCode,
};

/// An in-memory transport that records every request and replays a fixed
/// response.
#[derive(Clone, Debug, Default)]
struct MockBackend {
    response: Arc<Mutex<Option<HttpResponse>>>,
    requests: Arc<Mutex<Vec<HttpRequest>>>,
}

impl MockBackend {
    fn new(response: HttpResponse) -> Self {
        Self {
            response: Arc::new(Mutex::new(Some(response))),
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn requests(&self) -> Vec<HttpRequest> {
        self.requests.lock().unwrap().clone()
    }
}

impl AsyncTransport for MockBackend {
    type Error = std::io::Error;

    fn execute(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> + Send {
        self.requests.lock().unwrap().push(request);
        let response = self.response.lock().unwrap().clone().unwrap();
        async move { Ok(response) }
    }
}

impl BlockingTransport for MockBackend {
    type Error = std::io::Error;

    fn execute(&self, request: HttpRequest) -> Result<HttpResponse, Self::Error> {
        self.requests.lock().unwrap().push(request);
        Ok(self.response.lock().unwrap().clone().unwrap())
    }
}

fn json(body: &str) -> HttpResponse {
    HttpResponse::new(StatusCode::OK, body.as_bytes().to_vec())
}

const SERVER_INFO: &str =
    r#"{"servertime":1750134079,"servertimestring":"Mon Jun 16 21:21:19 2025"}"#;

#[tokio::test]
async fn async_request_builds_url_and_decodes() {
    let backend = MockBackend::new(json(SERVER_INFO));
    let client: Client<MockBackend, ApiKey> = Client::new(backend.clone(), ApiKey::new("SECRET"));

    let info = client.server_info().await.unwrap();
    assert_eq!(info.time_string, "Mon Jun 16 21:21:19 2025");

    let requests = backend.requests();
    assert_eq!(requests.len(), 1);
    let request = &requests[0];
    assert_eq!(request.method.as_str(), "GET");
    assert!(
        request
            .url
            .starts_with("https://api.steampowered.com/ISteamWebAPIUtil/GetServerInfo/v1?")
    );
    assert!(request.url.contains("key=SECRET"));
}

#[tokio::test]
async fn typed_query_parameters_are_encoded() {
    let backend = MockBackend::new(json(r#"{"response":{"success":true,"groups":[]}}"#));
    let client: Client<MockBackend, ApiKey> = Client::new(backend.clone(), ApiKey::new("KEY"));

    let groups = client
        .user(76_561_198_084_913_741u64)
        .groups()
        .await
        .unwrap();
    assert!(groups.is_empty());

    let url = &backend.requests()[0].url;
    assert!(url.contains("/ISteamUser/GetUserGroupList/v1?"));
    assert!(url.contains("steamid=76561198084913741"));
    assert!(url.contains("key=KEY"));
}

#[test]
fn blocking_request_decodes() {
    let backend = MockBackend::new(json(SERVER_INFO));
    let client: Client<MockBackend, Unauthenticated, Blocking> =
        Client::new(backend, Unauthenticated);

    let info = client.server_info().send().unwrap();
    assert_eq!(info.time_string, "Mon Jun 16 21:21:19 2025");
}

#[tokio::test]
async fn handles_are_static_and_send() {
    let backend = MockBackend::new(json(SERVER_INFO));
    let client: Client<MockBackend, ApiKey> = Client::new(backend, ApiKey::new("KEY"));

    // A `User` handle can outlive the expression that created it and be moved
    // into a spawned task, because it owns a cheap clone of the client rather
    // than borrowing it.
    let user = client.user(76_561_198_084_913_741u64);
    let id = tokio::spawn(async move { user.id() }).await.unwrap();
    assert_eq!(id.get(), 76_561_198_084_913_741);
}

#[tokio::test]
async fn http_error_status_is_reported() {
    let backend = MockBackend::new(HttpResponse::new(StatusCode::UNAUTHORIZED, b"no".to_vec()));
    let client: Client<MockBackend, Unauthenticated> = Client::new(backend, Unauthenticated);

    let error = client.server_info().await.unwrap_err();
    assert!(matches!(
        error,
        fume::Error::Http {
            status: StatusCode::UNAUTHORIZED,
            ..
        }
    ));
}
