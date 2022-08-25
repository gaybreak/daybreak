//! Discord objects and (de)serialization implementations on them

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
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding#interactions"
)]
pub mod interaction;
#[doc = discord_url!("https://discord.com/developers/docs/resources/guild#guild-member-object")]
pub mod member;
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#message-object")]
pub mod message;
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#presence")]
pub mod presence;
#[doc = discord_url!("https://discord.com/developers/docs/resources/user")]
pub mod user;
#[doc = discord_url!("https://discord.com/developers/docs/resources/voice#voice-resource")]
pub mod voice;
use enumflags2::bitflags;
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

#[bitflags]
#[repr(u64)]
#[derive(Clone, Copy, Debug)]
#[doc =discord_url!(
    "https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags"
)]
pub enum Permissions {
    CreateInstantInvite = 1 << 0,
    KickMembers = 1 << 1,
    BanMembers = 1 << 2,
    Administrator = 1 << 3,
    ManageChannels = 1 << 4,
    ManageGuild = 1 << 5,
    AddReactions = 1 << 6,
    ViewAuditLog = 1 << 7,
    PrioritySpeaker = 1 << 8,
    Stream = 1 << 9,
    ViewChannel = 1 << 10,
    SendMessages = 1 << 11,
    SendTtsMessages = 1 << 12,
    ManageMessages = 1 << 13,
    EmbedLinks = 1 << 14,
    AttachFiles = 1 << 15,
    ReadMessageHistory = 1 << 16,
    MentionEveryone = 1 << 17,
    UseExternalEmojis = 1 << 18,
    ViewGuildInsights = 1 << 19,
    Connect = 1 << 20,
    Speak = 1 << 21,
    MuteMembers = 1 << 22,
    DeafenMembers = 1 << 23,
    MoveMembers = 1 << 24,
    UseVad = 1 << 25,
    ChangeNickname = 1 << 26,
    ManageNicknames = 1 << 27,
    ManageRoles = 1 << 28,
    ManageWebhooks = 1 << 29,
    ManageEmojisAndStickers = 1 << 30,
    UseApplicationCommands = 1 << 31,
    RequestToSpeak = 1 << 32,
    ManageEvents = 1 << 33,
    ManageThreads = 1 << 34,
    CreatePublicThreads = 1 << 35,
    CreatePrivateThreads = 1 << 36,
    UseExternalStickers = 1 << 37,
    SendMessagesInThreads = 1 << 38,
    UseEmbeddedActivities = 1 << 39,
    ModerateMembers = 1 << 40,
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
