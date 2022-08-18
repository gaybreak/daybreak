//! Discord objects and (de)serialization implementations on them

#![allow(
    clippy::default_numeric_fallback,
    clippy::integer_arithmetic,
    clippy::integer_arithmetic,
    clippy::default_numeric_fallback,
    clippy::missing_docs_in_private_items
)]

#[doc = discord_url!("https://discord.com/developers/docs/reference#snowflakes")]
pub struct Id(u64);

impl Id {
    /// The unix timestamp of the ID
    #[doc = discord_url!(
        "https://discord.com/developers/docs/reference
        #snowflakes-snowflake-id-format-structure-left-to-right"
    )]
    pub fn timestamp(self) -> InternalResult<OffsetDateTime> {
        Ok(OffsetDateTime::from_unix_timestamp(
            ((self.0.checked_shr(22))
                .ok()?
                .checked_add(1_420_070_400_000))
            .ok()?
            .try_into()?,
        )?)
    }
}
