use once_cell::sync::Lazy;

use crate::http::Http;

/// Brings all of the stateful structs together
#[derive(Debug)]
pub struct Context(Lazy<ContextInner, fn(Config<'_>) -> ContextInner>);

#[derive(Debug)]
/// The actual context data wrapped to be wrapped with Lazy
struct ContextInner {
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
    /// use daybreak::context::{Context, Config};
    /// static CTX: Context = Context::new(&Config {
    ///     token: "my totally real token"
    /// });
    /// ```
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub const fn new(config: &Config<'_>) -> Self {
        Self(Lazy::new(|config| ContextInner {
            http: Http::new(config.token.to_owned()),
        }))
    }
}

/// The info required to create a context
#[derive(Clone, Debug)]
pub struct Config<'conf> {
    /// The bot's token as obtained from [Discord applications page](https://discord.com/developers/applications)
    pub token: &'conf str,
}
