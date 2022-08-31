use enumflags2::BitFlags;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::{context::Context, model::Permissions};

/// The client type used in this crate
pub type Http = Client<HttpsConnector<HttpConnector>>;
pub type HttpResult<T> = Result<T, HttpError>;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("The bot doesn't have the required permissions to make this request: {0}")]
    MissingPermissions(BitFlags<Permissions>),
    #[error(
        "Unexpected error, if this is an error with Daybreak, please open an issue at \
        https://github.com/gaybreak/daybreak/issues/new: {0}"
    )]
    Other(#[from] anyhow::Error),
}

/// Creates the HTTP client
pub fn create() -> Http {
    let connector = HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http2()
        .build();
    Client::builder().build(connector)
}

pub struct DiscordRequest<T: DeserializeOwned> {
    required_permissions: BitFlags<Permissions>,
    method: Method,
    endpoint: String,
    returns: T,
}

impl Context {
    async fn send_request<T: DeserializeOwned>(&self, request: DiscordRequest<T>) -> HttpResult<T> {
        if !self.permissions.contains(request.required_permissions) {
            return Err(HttpError::MissingPermissions(
                !(self.permissions & request.required_permissions),
            ));
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
                    .body(Body::empty())
                    .map_err(|err| HttpError::Other(err.into()))?,
            )
            .await
            .map_err(|err| HttpError::Other(err.into()))?
            .into_body();
        let bytes = to_bytes(body)
            .await
            .map_err(|err| HttpError::Other(err.into()))?;

        Ok(serde_json::from_str(
            std::str::from_utf8(&bytes).map_err(|err| HttpError::Other(err.into()))?,
        )
        .map_err(|err| HttpError::Other(err.into()))?)
    }
}
