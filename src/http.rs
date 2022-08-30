use hyper::{client::HttpConnector, Client};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

/// The client type used in this crate
#[derive(Debug)]
pub struct Http {
    /// The token to be used to connect to Discord
    token: String,
    /// The hyper client to make request
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Http {
    /// Creates a new HTTP client
    pub fn new(token: String) -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http2()
            .build();
        Self {
            token,
            client: Client::builder().build(connector),
        }
    }
}
