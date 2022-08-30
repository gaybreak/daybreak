use crate::http::{self, Http};

#[derive(Debug)]
/// Brings all of the stateful structs together
pub struct Context {
    /// The HTTP client used in the crate
    http: Http,
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
