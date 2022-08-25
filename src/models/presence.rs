use enumflags2::bitflags;
use time::OffsetDateTime;

use super::Id;

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-structure"
)]
pub struct Activity {
    pub name: String,
    pub kind: ActivityType,
    pub url: Option<String>,
    pub created_at: OffsetDateTime,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<Id>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<ActivityFlags>,
    pub buttons: Option<Vec<ActivityButton>>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-types"
)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-timestamps"
)]
pub struct ActivityTimestamps {
    pub start: Option<OffsetDateTime>,
    pub end: Option<OffsetDateTime>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-emoji"
)]
pub struct ActivityEmoji {
    pub name: String,
    pub id: Option<Id>,
    pub animated: Option<bool>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-party"
)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<(u8, u8)>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-assets"
)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-secrets"
)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub game: Option<String>,
}

#[bitflags]
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-flags"
)]
pub enum ActivityFlags {
    Instance = 1 << 0,
    Join = 1 << 1,
    Spectate = 1 << 2,
    JoinRequest = 1 << 3,
    Sync = 1 << 4,
    Play = 1 << 5,
    PartyPrivacyFriends = 1 << 6,
    PartyPrivacyVoiceChannel = 1 << 7,
    Embedded = 1 << 8,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#activity-object-activity-buttons"
)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}
