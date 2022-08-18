//! A sophisticated framework for the Discord API
//!
//! WIP

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::pattern_type_mismatch
)]

/// A result that shouldn't be an error, [please open an issue](NEW_ISSUE_URL)
/// if it is
///
/// [NEW_ISSUE_URL]: https://github.com/gaybreak/daybreak/issues/new
type InternalResult<T> = Result<T, anyhow::Error>;
