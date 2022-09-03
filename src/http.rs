use anyhow::Error;
use enumflags2::BitFlags;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request as HyperRequest};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::{de::DeserializeOwned, Serialize};

use crate::{model::permission::Permissions, Context, UserError};

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
    /// Send the given request to Discord with no body, returning the expected
    /// type
    #[doc = http_errors_doc!()]
    pub(crate) async fn empty_request<T: DeserializeOwned + Send>(
        &self,
        request: Request,
    ) -> Result<T, Error> {
        self.request(request, None::<()>).await
    }

    /// Send the given request to Discord with no body, returning the expected
    /// type
    #[doc = http_errors_doc!()]
    pub(crate) async fn request_with_params<T: DeserializeOwned + Send>(
        &self,
        request: Request,
        params: impl Serialize + Send,
    ) -> Result<T, Error> {
        self.request(request, Some(params)).await
    }

    /// Send the given request to Discord, returning the expected type
    #[doc = http_errors_doc!()]
    async fn request<T: DeserializeOwned + Send>(
        &self,
        request: Request,
        params: Option<impl Serialize + Send>,
    ) -> Result<T, Error> {
        if !self.permissions.contains(request.required_permissions) {
            return Err(UserError::MissingPermissions(
                !(self.permissions & request.required_permissions),
            )
            .into());
        }

        let request_body = if let Some(p) = params {
            serde_json::to_string(&p)?.into()
        } else {
            Body::empty()
        };

        let hyper_request = HyperRequest::builder()
            .method(request.method)
            .uri(format!("https://discord.com/api/v10{}", request.endpoint))
            .header(
                "User-Agent",
                "DiscordBot (https://github.com/gaybreak/daybreak 0.1)",
            )
            .header("Authorization", &self.token)
            .body(request_body)?;

        let response = self.http.request(hyper_request).await?.into_body();
        let bytes = to_bytes(response).await?;

        Ok(serde_json::from_str(std::str::from_utf8(&bytes)?)?)
    }
}
