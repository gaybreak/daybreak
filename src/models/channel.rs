use time::OffsetDateTime;

use super::{user::User, Id, message::MessageFlags};

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#channel-object-channel-structure"
)]
pub struct Channel<T> {
    pub id: Id,
    pub channel_type: u32,
    pub guild_id: Option<Id>,
    pub position: Option<u16>,
    pub permission_overwrites: Option<Vec<T>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Id>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u8>,
    pub rate_limit_per_user: Option<u16>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Id>,
    pub application_id: Option<Id>,
    pub parent_id: Option<Id>,
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,
    pub message_count: Option<u32>,
    pub member_count: Option<u8>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<T>,
    pub default_auto_archive_duration: Option<u16>,
    pub permissions: Option<T>,
    pub flags: Option<MessageFlags>,
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

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#channel-object-video-quality-modes"
)]
pub enum VideoQualityMode {
    Auto = 1,
    Full = 2,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!("https://discord.com/developers/docs/resources/channel#thread-metadata-object-thread-metadata-structure")]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u16,
    pub archive_timestamp: OffsetDateTime,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<OffsetDateTime>,
}
