use anyhow::Error;
use enumflags2::BitFlags;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request as HyperRequest};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::{de::DeserializeOwned, Serialize};

use crate::{model::permission::Permissions, Context, UserError};

/// The client type used in this crate
pub(crate) type Http = Client<HttpsConnector<HttpConnector>>;

/// Creates the HTTP client
pub(crate) fn create() -> Http {
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
    #[must_use]
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
    /// Send a custom request to Discord with no params, returning the expected
    /// type
    ///
    /// [Please open an issue](https://github.com/gaybreak/daybreak/issues/new)
    /// if you think the endpoint you're implementing is popular
    ///
    /// # Example
    ///
    /// Implements the [delete auto moderation rule endpoint][DISCORD_DOCS_URL]
    ///
    /// [DISCORD_DOCS_URL]:
    /// https://discord.com/developers/docs/resources/auto-moderation#delete-auto-moderation-rule
    ///
    /// ```rust
    /// use anyhow::Error;
    /// use daybreak::{
    ///     http::Request,
    ///     model::{
    ///         auto_moderation::{
    ///             AutoModerationAction, AutoModerationEvent, AutoModerationRule,
    ///             AutoModerationTrigger,
    ///         },
    ///         permission::Permissions,
    ///         Id,
    ///     },
    ///     Context, ContextConfig,
    /// };
    /// use hyper::Method;
    /// use once_cell::sync::Lazy;
    /// use serde::Serialize;
    ///
    /// static CTX: Lazy<Context> = Lazy::new(|| {
    ///     Context::new(&ContextConfig {
    ///         token: env!("DAYBREAK_BOT_TOKEN"),
    ///     })
    /// });
    ///
    /// async fn delete_auto_moderation_rule(
    ///     guild_id: Id,
    ///     auto_moderation_rule_id: Id,
    /// ) -> Result<(), Error> {
    ///     CTX.empty_request(Request::new(
    ///         Permissions::ManageGuild.into(),
    ///         Method::DELETE,
    ///         format!("/guilds/{guild_id}/auto-moderation/rules/{auto_moderation_rule_id}"),
    ///     ))
    ///     .await
    /// }
    /// ```
    #[doc = http_errors_doc!()]
    pub async fn empty_request<T: DeserializeOwned + Send>(
        &self,
        request: Request,
    ) -> Result<T, Error> {
        self.request(request, None::<()>).await
    }

    /// Send a custom request to Discord with the given JSON params, returning
    /// the expected type
    ///
    /// [Please open an issue](https://github.com/gaybreak/daybreak/issues/new)
    /// if you think the endpoint you're implementing is popular
    ///
    /// # Example
    ///
    /// Implements the [modify auto moderation rule endpoint][DISCORD_DOCS_URL]
    ///
    /// [DISCORD_DOCS_URL]:
    /// https://discord.com/developers/docs/resources/auto-moderation#modify-auto-moderation-rule
    ///
    /// ```rust
    /// use anyhow::Error;
    /// use daybreak::{
    ///     http::Request,
    ///     model::{
    ///         auto_moderation::{
    ///             AutoModerationAction, AutoModerationEvent, AutoModerationRule,
    ///             AutoModerationTrigger,
    ///         },
    ///         permission::Permissions,
    ///         Id,
    ///     },
    ///     Context, ContextConfig,
    /// };
    /// use hyper::Method;
    /// use once_cell::sync::Lazy;
    /// use serde::Serialize;
    ///
    /// static CTX: Lazy<Context> = Lazy::new(|| {
    ///     Context::new(&ContextConfig {
    ///         token: env!("DAYBREAK_BOT_TOKEN"),
    ///     })
    /// });
    ///
    /// #[derive(Serialize)]
    /// struct EditAutoModerationRule {
    ///     name: Option<String>,
    ///     event_type: Option<AutoModerationEvent>,
    ///     trigger_metadata: Option<AutoModerationTrigger>,
    ///     actions: Option<Vec<AutoModerationAction>>,
    ///     enabled: Option<bool>,
    ///     exempt_roles: Option<Vec<Id>>,
    ///     exempt_channels: Option<Vec<Id>>,
    /// }
    ///
    /// impl EditAutoModerationRule {
    ///     async fn exec(
    ///         self,
    ///         guild_id: Id,
    ///         auto_moderation_rule_id: Id,
    ///     ) -> Result<AutoModerationRule, Error> {
    ///         let request = Request::new(
    ///             Permissions::ManageGuild.into(),
    ///             Method::PATCH,
    ///             format!("/guilds/{guild_id}/auto-moderation/rules/{auto_moderation_rule_id}"),
    ///         );
    ///         CTX.request_with_params(request, self).await
    ///     }
    /// }
    /// ```
    #[doc = http_errors_doc!()]
    pub async fn request_with_params<T: DeserializeOwned + Send>(
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
