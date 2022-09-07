use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

use super::{user::User, Id};

#[doc =discord_url!(
    "https://discord.com/developers/docs/resources/guild-scheduled-event\
    #guild-scheduled-event-object-guild-scheduled-event-structure"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduledEvent {
    pub id: Id,
    pub guild_id: Id,
    pub channel_id: Option<Id>,
    pub creator_id: Option<Id>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_start_time: OffsetDateTime,
    pub scheduled_end_time: Option<OffsetDateTime>,
    pub privacy_level: ScheduledEventPrivacy,
    pub status: ScheduledEventStatus,
    pub entity_type: ScheduledEventEntityType,
    pub entity_id: Option<Id>,
    pub entity_metadata: ScheduledEventEntity,
    pub creator: Option<User>,
    pub user_count: Option<u16>,
    pub image: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild-scheduled-event\
    #guild-scheduled-event-object-guild-scheduled-event-privacy-level"
)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum ScheduledEventPrivacy {
    GuildOnly = 2,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild-scheduled-event\
    #guild-scheduled-event-object-guild-scheduled-event-status"
)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum ScheduledEventStatus {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild-scheduled-event\
    #guild-scheduled-event-object-guild-scheduled-event-entity-metadata"
)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledEventEntity {
    pub location: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild-scheduled-event\
    #guild-scheduled-event-object-guild-scheduled-event-entity-types"
)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum ScheduledEventEntityType {
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-scheduled-event-user-add-guild-scheduled-event-user-add-event-fields"
)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-scheduled-event-user-remove-guild-scheduled-event-user-remove-event-fields"
)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ScheduledEventUser {
    pub guild_scheduled_event_id: Id,
    pub user_id: Id,
    pub guild_id: Id,
}
