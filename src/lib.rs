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

/// Context method on auto moderation
mod auto_moderation;
/// Context method for channels
mod channel;
/// Implementation of the HTTP client to make requests to Discord
pub mod http;
/// Discord objects and (de)serialization implementations on them
pub mod model;
/// Tests for everything in Daybreak
#[cfg(test)]
mod tests;
/// Message impl
mod message;

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
    /// use daybreak::{model::Id, Context, ContextConfig};
    /// use once_cell::sync::Lazy;
    ///
    /// static CTX: Lazy<Context> = Lazy::new(|| {
    ///     Context::new(&ContextConfig {
    ///         token: "my totally real token",
    ///     })
    /// });
    ///
    /// CTX.auto_moderation_rules(Id(1234));
    /// ```
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(config: &ContextConfig<'_>) -> Self {
        Self {
            token: format!("Bot {}", config.token),
            permissions: Permissions::all(),
            http: http::create(),
        }
    }
}

/// The info required to create a context
#[derive(Clone, Debug)]
pub struct ContextConfig<'conf> {
    /// The bot's token as obtained from
    /// [Discord applications page](https://discord.com/developers/applications)
    pub token: &'conf str,
}

/// A user-facing error
///
/// This should have every error to be reported to the user of the bot,
/// so you can add your own errors to this enum, refer to [`Self::custom`] to
/// see how to do that
///
/// You won't receive these in an interaction event, where they're handled
/// implicitly for you!
///
/// In other events, you'll just get an `anyhow::Error` which you can downcast
/// to see if it's a user error, you don't have to do this if you or the methods
/// you call never return a `UserError`
///
/// # Example
///
/// ```rust
/// use daybreak::UserError;
/// # let my_amazing_error = anyhow::anyhow!("");
/// # fn tell_the_user_somehow(_: String) {};
///
/// match my_amazing_error.downcast::<UserError>() {
///     Ok(user_err) => tell_the_user_somehow(user_err.to_string()),
///     Err(err) => eprintln!("The dev really messed this up huh: {err}"),
/// }
/// ```
///
/// # Error Handling Reasoning
///
/// Daybreak methods return an `anyhow::Error` because it usually means a bug in
/// your program or Daybreak, and the best way to handle those is by printing
/// them and notifying the developer
///
/// If you think the returned error is an problem in Daybreak,
/// [please open an issue](https://github.com/gaybreak/daybreak/issues/new)
#[derive(Error, Debug)]
pub enum UserError {
    /// The bot is missing some permissions
    #[error(
        "Please give the bot these permissions:\n{}",
        permission::to_pretty_string(*.0),
    )]
    MissingPermissions(BitFlags<Permissions>),
    /// The error is user-defined
    #[error("{0}")]
    Custom(anyhow::Error),
}

impl UserError {
    /// Create a custom user error from any error type
    ///
    /// ```rust
    /// use anyhow::Error;
    /// use daybreak::UserError;
    ///
    /// #[derive(thiserror::Error, Debug)]
    /// enum CustomUserError {
    ///     #[error(
    ///         "You're cringe because your username is *{0}*, cringe people aren't allowed to use \
    ///          this bot"
    ///     )]
    ///     MemberCringe(String),
    ///     // Other user errors here
    /// }
    ///
    /// fn cringe_check(member_nick: String) -> Result<(), Error> {
    ///     if member_nick.contains("69") {
    ///         return Err(UserError::custom(CustomUserError::MemberCringe(member_nick)).into());
    ///     }
    ///     Ok(())
    /// }
    ///
    /// assert_eq!(
    ///     cringe_check("your-mom-69".to_owned())
    ///         .unwrap_err()
    ///         .to_string(),
    ///     "You're cringe because your username is *your-mom-69*, cringe people aren't allowed to \
    ///      use this bot",
    /// )
    /// ```
    pub fn custom(err: impl Into<anyhow::Error>) -> Self {
        Self::Custom(err.into())
    }
}
