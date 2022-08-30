use enumflags2::{BitFlag, BitFlags};

use crate::{
    http::{self, Http},
    model::Permissions,
};

// /// Context methods about the audit log
// pub mod audit_log;
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #list-auto-moderation-rules-for-guild"
)]
pub mod auto_moderation;

#[derive(Debug)]
/// Brings all of the stateful structs together
pub struct Context {
    /// The bot's token, unlike `Config.token`, this has `Bot ` prepended to it
    pub(crate) token: String,
    /// The permissions the bot has, this will be removed at release
    pub(crate) permissions: BitFlags<Permissions>,
    /// The HTTP client used in the crate
    pub(crate) http: Http,
}

impl Context {
    /// Create a context from a given config
    ///
    /// This is probably the first method you'll need to call to use this
    /// framework so welcome to Daybreak!
    ///
    /// # Example
    /// ```rust
    /// use daybreak::context::{Config, Context};
    /// use once_cell::sync::Lazy;
    ///
    /// static CTX: Lazy<Context> = Lazy::new(|| {
    ///     Context::new(&Config {
    ///         token: "my totally real token",
    ///     })
    /// });
    /// ```
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(config: &Config<'_>) -> Self {
        Self {
            token: format!("Bot {}", config.token),
            permissions: Permissions::all(),
            http: http::create(),
        }
    }
}

/// The info required to create a context
#[derive(Clone, Debug)]
pub struct Config<'conf> {
    /// The bot's token as obtained from
    /// [Discord applications page](https://discord.com/developers/applications)
    pub token: &'conf str,
}
