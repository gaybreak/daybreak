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
    unreachable_pub,
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
    clippy::exhaustive_enums
)]

/// To be used like `#[doc = discord_url!("https://discord.com/developers/docs/...")]`
macro_rules! discord_url {
    ($url: literal) => {
        concat!("\n\n[Refer to the Discord docs](", $url, ")")
    };
}

use discord_url;

pub mod models;

/// A result that shouldn't be an error, [please open an issue](NEW_ISSUE_URL)
/// if it is
///
/// [NEW_ISSUE_URL]: https://github.com/gaybreak/daybreak/issues/new
type InternalResult<T> = Result<T, anyhow::Error>;
