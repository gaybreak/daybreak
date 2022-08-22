//! Discord objects and (de)serialization implementations on them

#![allow(
    clippy::default_numeric_fallback,
    clippy::integer_arithmetic,
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    missing_docs
)]

#[doc = discord_url!("https://discord.com/developers/docs/resources/application")]
pub mod application;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#attachment-object")]
pub mod attachment;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#channel-object")]
pub mod channel;
#[doc = discord_url!("https://discord.com/developers/docs/interactions/application-commands")]
pub mod commands;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#embed-object")]
pub mod embed;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#message-object")]
pub mod message;
#[doc = discord_url!("https://discord.com/developers/docs/topics/teams")]
pub mod teams;
#[doc = discord_url!("https://discord.com/developers/docs/resources/user")]
pub mod user;

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
    #[allow(
        clippy::integer_arithmetic,
        clippy::cast_possible_wrap,
        clippy::as_conversions
    )]
    pub fn timestamp(self) -> InternalResult<OffsetDateTime> {
        Ok(OffsetDateTime::from_unix_timestamp(
            Duration::milliseconds(((self.0 >> 22) + 1_420_070_400_000) as i64).whole_seconds(),
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
