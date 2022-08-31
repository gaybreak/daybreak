use anyhow::Error;
use enumflags2::BitFlags;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request as HyperRequest};
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
#[derive(Clone, Debug)]
pub struct Request {
    /// The required permissions to make the request
    required_permissions: BitFlags<Permissions>,
    /// The method of the request
    method: Method,
    /// The endpoint URL of the request
    endpoint: String,
}

impl Request {
    /// Creates a new `Request` from the given fields
    pub const fn new(
        required_permissions: BitFlags<Permissions>,
        method: Method,
        endpoint: String,
    ) -> Self {
        Self {
            required_permissions,
            method,
            endpoint,
        }
    }
}

impl Context {
    /// Send the given request to Discord, returning the expected type
    #[doc = http_errors_doc!()]
    pub async fn request<T: DeserializeOwned + Send>(&self, request: Request) -> Result<T, Error> {
        if !self.permissions.contains(request.required_permissions) {
            return Err(UserError::MissingPermissions(
                !(self.permissions & request.required_permissions),
            )
            .into());
        }

        let body = self
            .http
            .request(
                HyperRequest::builder()
                    .method(request.method)
                    .uri(format!("https://discord.com/api/v10{}", request.endpoint))
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
