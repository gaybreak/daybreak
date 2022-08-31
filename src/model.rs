#![allow(
    clippy::default_numeric_fallback,
    clippy::integer_arithmetic,
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    missing_docs
)]

#[doc = discord_url!("https://discord.com/developers/docs/resources/application")]
pub mod application;
// #[doc = discord_url!(
// "https://discord.com/developers/docs/resources/audit-log#audit-logs-resource"
// )]
// pub mod audit_log;
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object"
)]
pub mod auto_moderation;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#channel-object")]
pub mod channel;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#embed-object")]
pub mod embed;
#[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#emoji-object")]
pub mod emoji;
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#connecting-and-resuming")]
pub mod gateway;
#[doc = discord_url!("https://discord.com/developers/docs/resources/guild#guild-resource")]
pub mod guild;
#[doc = discord_url!("https://discord.com/developers/docs/resources/guild#integration-object")]
pub mod integration;
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding#interactions"
)]
pub mod interaction;
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#invites")]
pub mod invite;
#[doc = discord_url!("https://discord.com/developers/docs/resources/guild#guild-member-object")]
pub mod member;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#message-object")]
pub mod message;
#[doc =discord_url!("https://discord.com/developers/docs/topics/permissions")]
pub mod permission;
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#presence")]
pub mod presence;
#[doc = discord_url!("https://discord.com/developers/docs/topics/permissions#role-object")]
pub mod role;
#[doc = discord_url!("https://discord.com/developers/docs/resources/guild-scheduled-event")]
pub mod scheduled_event;
#[doc = discord_url!("https://discord.com/developers/docs/resources/user")]
pub mod user;
#[doc = discord_url!("https://discord.com/developers/docs/resources/voice#voice-resource")]
pub mod voice;
// #[doc = discord_url!("https://discord.com/developers/docs/resources/webhook#webhook-resource")]
// pub mod webhook;

use anyhow::Error;
use time::{Duration, OffsetDateTime};

#[doc = discord_url!("https://discord.com/developers/docs/reference#snowflakes")]
#[derive(Clone, Copy, Debug)]
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
    pub fn timestamp(self) -> Result<OffsetDateTime, Error> {
        Ok(OffsetDateTime::from_unix_timestamp(
            Duration::milliseconds(((self.0 >> 22) + 1_420_070_400_000) as i64).whole_seconds(),
        )?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
