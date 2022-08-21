use time::OffsetDateTime;

use super::{user::User, Id};

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#channel-object-channel-structure"
)]
pub struct Channel<T> {
    pub id: Id,
    pub channel_type: u32,
    pub guild_id: Option<Id>,
    pub position: Option<u32>,
    pub permission_overwrites: Option<T>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Id>,
    pub bitrate: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Id>,
    pub application_id: Option<Id>,
    pub parent_id: Option<Id>,
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u32>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub thread_metadata: Option<T>,
    pub member: Option<T>,
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<u32>,
    pub total_message_sent: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#channel-object-channel-types"
)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-structure"
)]
pub struct Embed {
    pub title: Option<String>,
    pub embed_type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<OffsetDateTime>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure"
)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure"
)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure"
)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure"
)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure"
)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure"
)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure"
)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
