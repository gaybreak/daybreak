use time::OffsetDateTime;

use super::{member::ThreadMember, user::User, Id, Permissions};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#channel-object-channel-structure"
)]
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#thread-create")]
#[derive(Clone, Debug)]
pub struct Channel<T> {
    pub id: Id,
    pub channel_type: ChannelType,
    pub guild_id: Option<Id>,
    pub position: Option<u16>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
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
    pub thread_metadata: Option<Thread>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u16>,
    pub permissions: Option<Permissions>,
    pub flags: Option<T>,
    pub total_message_sent: Option<u32>,
    #[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#thread-create")]
    pub newly_created: Option<bool>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#channel-object-channel-types"
)]
#[derive(Clone, Copy, Debug)]
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

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#channel-object-video-quality-modes"
)]
#[derive(Clone, Copy, Debug)]
pub enum VideoQualityMode {
    Auto = 1,
    Full = 2,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #thread-metadata-object-thread-metadata-structure"
)]
#[derive(Clone, Copy, Debug)]
pub struct Thread {
    pub archived: bool,
    pub auto_archive_duration: u16,
    pub archive_timestamp: OffsetDateTime,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<OffsetDateTime>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #channel-pins-update-channel-pins-update-event-fields"
)]
#[derive(Clone, Copy, Debug)]
pub struct ChannelPinsUpdate {
    pub guild_id: Option<Id>,
    pub channel_id: Id,
    pub last_pin_timestamp: Option<OffsetDateTime>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #thread-list-sync-thread-list-sync-event-fields"
)]
#[derive(Clone, Debug)]
pub struct ThreadSync<T> {
    pub guild_id: Id,
    pub channel_ids: Option<Vec<Id>>,
    pub threads: Vec<Channel<T>>,
    pub members: Vec<ThreadMember>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#overwrite-object-overwrite-structure"
)]
#[derive(Clone, Debug)]
pub struct PermissionOverwrite {
    pub id: Id,
    pub kind: PermissionOverwriteKind,
    pub allow: String,
    pub deny: String,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#overwrite-object-overwrite-structure"
)]
#[derive(Clone, Copy, Debug)]
pub enum PermissionOverwriteKind {
    Role = 0,
    Member = 1,
}
