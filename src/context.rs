use once_cell::sync::Lazy;

use crate::http::{self, Http};

/// Brings all of the stateful structs together
#[derive(Debug)]
pub struct Context(Lazy<ContextInner>);

#[derive(Debug)]
/// The actual context data wrapped to be wrapped with Lazy
struct ContextInner {
    /// The HTTP client used in the crate
    http: Http,
}

impl Context {
    /// Create a context from a given config
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
        Self(Lazy::new(|| ContextInner {
            http: http::create(),
        }))
    }
}

/// The info required to create a context
#[derive(Clone, Debug)]
pub struct Config<'conf> {
    /// The bot's token as obtained from [Discord applications page](https://discord.com/developers/applications)
    pub token: &'conf str,
}
