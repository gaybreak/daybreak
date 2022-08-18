//! A sophisticated framework for the Discord API
//!
//! WIP

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::self_named_module_files,
    clippy::separated_literal_suffix,
    clippy::missing_inline_in_public_items,
    clippy::exhaustive_structs,
    clippy::exhaustive_enums
)]

/// To be used like `#[doc = discord_url!("https://discord.com/developers/docs/...")]`
macro_rules! discord_url {
    ($url: literal) => {
        concat!("\n\n[Refer to the Discord docs](", $url, ")")
    };
}

use discord_url;

/// A result that shouldn't be an error, [please open an issue](NEW_ISSUE_URL)
/// if it is
///
/// [NEW_ISSUE_URL]: https://github.com/gaybreak/daybreak/issues/new
type InternalResult<T> = Result<T, anyhow::Error>;
