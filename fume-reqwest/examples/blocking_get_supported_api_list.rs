use std::time::Duration;

use fume::{ApiKey, Client, Unauthenticated};
use fume_reqwest::ReqwestBlockingBackend;

fn main() -> anyhow::Result<()> {
    let http = reqwest::blocking::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;
    let backend = ReqwestBlockingBackend::with_client(http);

    match std::env::args().nth(1) {
        Some(key) => {
            let client = Client::new(backend, ApiKey::new(key));
            let apis = client.apis().send()?;
            println!("{} interfaces (authenticated)", apis.len());
        }
        None => {
            let client = Client::new(backend, Unauthenticated);
            let apis = client.apis().send()?;
            println!("{} interfaces (public)", apis.len());
        }
    }

    Ok(())
}
