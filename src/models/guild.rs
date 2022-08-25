use enumflags2::bitflags;
use time::OffsetDateTime;

use super::{channel::Channel, voice::VoiceState, Id, Permissions};

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-object-guild-structure"
)]
pub struct Guild<T> {
    pub id: Id,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: Id,
    pub permissions: Option<Permissions>,
    pub region: Option<String>,
    pub afk_channel_id: Option<Id>,
    pub afk_timeout: u16,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<Id>,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: MessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub roles: Vec<T>,
    pub emojis: Vec<T>,
    pub features: Vec<String>,
    pub mfa_level: MfaLevel,
    pub application_id: Option<Id>,
    pub system_channel_id: Option<Id>,
    pub system_channel_flags: SystemChannelFlags,
    pub rules_channel_id: Option<Id>,
    pub max_presences: Option<u32>,
    pub max_members: Option<u32>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: PremiumTier,
    pub premium_subscription_count: Option<u16>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<Id>,
    pub max_video_channel_users: Option<u16>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: NsfwLevel,
    pub stickers: Vec<T>,
    pub premium_progress_bar_enabled: bool,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway#guild-create-guild-create-extra-fields"
    )]
    pub joined_at: Option<OffsetDateTime>,
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<u32>,
    pub voice_states: Option<Vec<VoiceState<T>>>,
    pub members: Option<Vec<T>>,
    pub channels: Option<Vec<Channel<T>>>,
    pub threads: Option<Vec<Channel<T>>>,
    pub presences: Option<Vec<T>>,
    pub stage_instances: Option<Vec<StageInstance>>,
    pub guild_scheduled_events: Option<Vec<T>>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-object-verification-level"
)]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild\
    #guild-object-default-message-notification-level"
)]
pub enum MessageNotificationLevel {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level"
)]
pub enum ExplicitContentFilterLevel {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!("https://discord.com/developers/docs/resources/guild#guild-object-mfa-level")]
pub enum MfaLevel {
    None = 0,
    Elevated = 1,
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[doc = discord_url!("https://discord.com/developers/docs/resources/guild#guild-object-mfa-level")]
pub enum SystemChannelFlags {
    SuppressJoinNotifications = 1 << 0,
    SuppressPremiumSubscriptions = 1 << 1,
    SuppressGuildReminderNotifications = 1 << 2,
    SuppressJoinNotificationReplies = 1 << 3,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-object-premium-tier"
)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild\
    #welcome-screen-object-welcome-screen-structure"
)]
pub struct WelcomeScreen {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild\
    #welcome-screen-object-welcome-screen-channel-structure"
)]
pub struct WelcomeScreenChannel {
    pub channel_id: Id,
    pub description: String,
    pub emoji_id: Option<Id>,
    pub emoji_name: Option<String>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-object-guild-nsfw-level"
)]
pub enum NsfwLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/stage-instance\
    #stage-instance-object-stage-instance-structure"
)]
pub struct StageInstance {
    pub id: Id,
    pub guild_id: Id,
    pub channel_id: Id,
    pub topic: String,
    pub privacy_level: StagePrivacyLevel,
    pub discoverable_disabled: bool,
    pub guild_scheduled_event_id: Option<Id>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/stage-instance\
    #stage-instance-object-privacy-level"
)]
pub enum StagePrivacyLevel {
    Public = 1,
    GuildOnly = 2,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#unavailable-guild-object"
)]
pub struct UnavailableGuild {
    pub id: Id,
    pub unavailable: bool,
}
