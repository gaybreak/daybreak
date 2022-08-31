use anyhow::Error;
use enumflags2::BitFlags;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::de::DeserializeOwned;

use crate::{context::Context, model::permission::Permissions, UserError};

/// The client type used in this crate
pub type Http = Client<HttpsConnector<HttpConnector>>;

/// Creates the HTTP client
pub fn create() -> Http {
    let connector = HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http2()
        .build();
    Client::builder().build(connector)
}

/// An HTTP request to be made to the Discord HTTP API
pub struct DiscordRequest<T: DeserializeOwned> {
    /// The required permissions to make the request
    required_permissions: BitFlags<Permissions>,
    /// The method of the request
    method: Method,
    /// The endpoint URL of the request
    endpoint: String,
    /// The type to deserialize the response to
    returns: T,
}

impl Context {
    /// Send a `DiscordRequest`, returning the expected type
    async fn send_request<T: DeserializeOwned + Send>(
        &self,
        request: DiscordRequest<T>,
    ) -> Result<T, Error> {
        if !self.permissions.contains(request.required_permissions) {
            return Err(UserError::MissingPermissions(
                !(self.permissions & request.required_permissions),
            )
            .into());
        }

        let body = self
            .http
            .request(
                Request::builder()
                    .method(request.method)
                    .uri(format!("https://discord.com/api/v10/{}", request.endpoint))
                    .header(
                        "User-Agent",
                        "DiscordBot (https://github.com/gaybreak/daybreak 0.1)",
                    )
                    .header("Authorization", &self.token)
                    .body(Body::empty())?,
            )
            .await?
            .into_body();
        let bytes = to_bytes(body).await?;

        Ok(serde_json::from_str(std::str::from_utf8(&bytes)?)?)
    }
}
