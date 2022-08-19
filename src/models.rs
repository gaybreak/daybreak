//! Discord objects and (de)serialization implementations on them

#![allow(
    clippy::default_numeric_fallback,
    clippy::integer_arithmetic,
    clippy::integer_arithmetic,
    clippy::default_numeric_fallback,
    clippy::missing_docs_in_private_items,
    missing_docs
)]

#[doc = discord_url!("https://discord.com/developers/docs/resources/application")]
pub mod application;
#[doc = discord_url!("https://discord.com/developers/docs/topics/teams")]
pub mod teams;
#[doc = discord_url!("https://discord.com/developers/docs/resources/user")]
pub mod user;

use anyhow::IntoResult;
use time::{Duration, OffsetDateTime};

use crate::{discord_url, InternalResult};

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!("https://discord.com/developers/docs/reference#snowflakes")]
pub struct Id(pub u64);

impl Id {
    /// The unix timestamp of the ID
    #[doc = discord_url!(
        "https://discord.com/developers/docs/reference\
        #snowflakes-snowflake-id-format-structure-left-to-right"
    )]
    pub fn timestamp(self) -> InternalResult<OffsetDateTime> {
        Ok(OffsetDateTime::from_unix_timestamp(
            Duration::milliseconds(
                (self.0.checked_shr(22))
                    .ok()?
                    .checked_add(1_420_070_400_000)
                    .ok()?
                    .try_into()?,
            )
            .whole_seconds(),
        )?)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use super::Id;

    #[test]
    fn id_timestamp() {
        assert_eq!(
            Id(258_568_289_746_288_641).timestamp().unwrap(),
            OffsetDateTime::from_unix_timestamp(1_481_717_884).unwrap()
        );
    }
}
