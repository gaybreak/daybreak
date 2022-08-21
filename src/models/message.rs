use time::OffsetDateTime;

use super::{user::User, channel::Channel, Id};
use crate::models::application::Application;

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#message-object-message-structure"
)]
pub struct Message<T> {
    pub id: Id,
    pub channel_id: Id,
    pub author: User,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub edited_at: Option<OffsetDateTime>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<Id>,
    pub attachments: Vec<T>,
    pub embeds: Vec<T>,
    pub reactions: Option<Vec<T>>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<Id>,
    pub message_type: T,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<Id>,
    pub message_reference: Option<MessageReference>,
    pub flags: T,
    pub referenced_message: Option<Box<Message<T>>>,
    pub interaction: Option<T>,
    pub thread: Option<T>,
    pub components: Option<Vec<T>>,
    pub sticker_items: Option<Vec<T>>,
    pub stickers: Option<Vec<T>>,
    pub position: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#message-reference-object"
)]
pub struct MessageReference {
    pub message_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub guild_id: Option<Id>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel
    #message-object-message-activity-structure"
)]
pub struct MessageActivity {
    pub message_activity_type: MessageActivityType,
    pub party_id: Option<String>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#message-object-message-activity-types"
)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#message-object-message-types"
)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
}