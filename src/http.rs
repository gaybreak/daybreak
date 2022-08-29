use hyper::{client::HttpConnector, Client};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

/// The client type used in this crate
pub type Http = Client<HttpsConnector<HttpConnector>>;

/// Creates a new http client
pub fn create() -> Http {
    let connector = HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http2()
        .build();
    Client::builder().build(connector)
}
