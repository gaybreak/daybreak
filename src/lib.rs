//! A sophisticated framework for the Discord API
//!
//! WIP

#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    // unstable lints:
    // unreachable_pub,
    // nightly lints:
    // fuzzy_provenance_casts,
    // lossy_provenance_casts,
    // must_not_suspend,
    // non_exhaustive_omitted_patterns,
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::self_named_module_files,
    clippy::separated_literal_suffix,
    clippy::missing_inline_in_public_items,
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    // Disallow before release:
    dead_code,
)]

use enumflags2::{BitFlag, BitFlags};
use http::Http;
use model::permission::{self, Permissions};
use thiserror::Error;

/// # Example
/// ```ignore
/// #[doc = discord_url!("https://discord.com/developers/docs/...")]
/// ```
macro_rules! discord_url {
    ($url: literal) => {
        concat!("\n\n[Refer to the Discord docs](", $url, ")")
    };
}

/// # Example
/// ```ignore
/// #[doc = fields_documented!()]
/// ```
macro_rules! fields_documented {
    () => {
        "Refer to the documentation of the fields"
    };
}

/// # Example
/// ```ignore
/// #[doc = variants_documented!()]
/// ```
macro_rules! variants_documented {
    () => {
        "Refer to the documentation of the variants"
    };
}

/// # Example
/// ```ignore
/// #[doc = discord_url!("https://discord.com/developers/docs/...")]
/// #[doc = http_errors_doc!()]
/// ```
macro_rules! http_errors_doc {
    () => {
        "# Errors\n\nReturns [`crate::UserError::MissingPermissions`] when the bot doesn't have \
         the required permissions\n\nOr an `anyhow::Error` on an HTTP or deserialization error"
    };
}

/// Implementation of the HTTP client to make requests to Discord
mod http;
/// Discord objects and (de)serialization implementations on them
pub mod model;

// /// Context methods about the audit log
// pub mod audit_log;
/// Context method on auto moderation
mod auto_moderation;

#[derive(Debug)]
#[allow(clippy::multiple_inherent_impl)]
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

/// A user-facing error
///
/// This is the only error type used in Daybreak by design because in most cases
/// only the type of the user-facing error is useful and the rest is handled the
/// same by printing, executing a webhook etc.
///
/// This means the simplest way to handle errors in your code flow is using
/// `anyhow::Error` everywhere and downcasting it when it might be a user error
///
/// If this is an error with Daybreak,
/// [please open an issue](https://github.com/gaybreak/daybreak/issues/new)
///
/// # Downcasting Example
///
/// ```rust
/// use std::fs;
///
/// use anyhow::{anyhow, Error};
/// use daybreak::UserError;
///
/// fn maybe_user_error(user_input: &[u8]) -> Result<(), Error> {
///     if std::str::from_utf8(user_input)?.starts_with("Boo!") {
///         // Read the next example to see how to actually use `UserError`
///         return Err(UserError::custom(anyhow!("Your input scared me :(")).into());
///     }
///     Ok(())
/// }
///
/// assert!(maybe_user_error(&[159]) // Invalid byte
///     .unwrap_err()
///     .downcast_ref::<UserError>()
///     .is_none());
/// assert!(maybe_user_error(&[66, 111, 111, 33]) // Bytes for "Boo!"
///     .unwrap_err()
///     .downcast_ref::<UserError>()
///     .is_some())
/// ```
///
/// This should have every error to be reported to the user of the bot,
/// so you can add your own errors to this enum
///
/// # User Error Example
///
/// ```rust
/// use daybreak::UserError;
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// enum CoolCommandError {
///     #[error("You are cringe, cringe people are not allowed to use this bot")]
///     MemberCringe,
///     // Other user errors here
/// }
///
/// fn check_for_cringe(member_nick: &str) -> Result<(), UserError> {
///     if member_nick.contains("69") {
///         return Err(UserError::custom(CoolCommandError::MemberCringe));
///     }
///     return Ok(());
/// }
///
/// assert_eq!(
///     check_for_cringe("your-mom-69").unwrap_err().to_string(),
///     "You are cringe, cringe people are not allowed to use this bot",
/// )
/// ```
#[derive(Error, Debug)]
pub enum UserError {
    /// The bot is missing some permissions
    #[error(
        "The bot doesn't have the required permissions for this:\n{}",
        permission::to_pretty_string(*.0),
    )]
    MissingPermissions(BitFlags<Permissions>),
    /// The error is user-defined
    #[error("{0}")]
    Custom(anyhow::Error),
}

impl UserError {
    /// Create a custom user error from any error type
    pub fn custom(err: impl Into<anyhow::Error>) -> Self {
        Self::Custom(err.into())
    }
}
