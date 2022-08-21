use super::{channel::Channel, user::User, Id};
use crate::models::application::Application;

#[derive(Clone, Debug)]
#[doc = discord_url!(
   "https://discord.com/developers/docs/resources/channel#message-object-message-structure"
)]

pub struct Message {
    pub id: Id,
    pub channel_id: Id,
    pub author: User,
    pub content: String,
    pub created_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<T>,
    pub attachments: Vec<T>,
    pub embeds: Vec<T>,
    pub reactions: Option<Vec<T>>,
    pub nonce: Option<u32>,
    pub pinned: bool,
    pub webhook_id: Option<Id>,
    pub message_type: u32,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<Id>,
    pub message_reference: Option<MessageReference>,
    pub flags: u32,
    pub referenced_message: Option<Message>, // only returned for 19 (REPLY) and 21 (THREAD_STARTER_MESSAGE)
    pub interaction: Option<T>,
    pub thread: Option<Channel>,
    pub components: Option<Vec<T>>,
    pub sticker_items: Option<Vec<T>>,
    pub stickers: Option<T>,
    pub position: Option<u32>,
}

pub struct MessageReference {
    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-reference-object"
    )]
    pub message_id: Option<Id>, // ID of original message
    pub channel_id: Option<Id>, // Channel id is only optional when creating a reply
    pub guild_id: Option<Id>,
    pub fail_if_not_exists: Option<bool>,
}

pub struct MessageActivity {
    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-object-message-activity-structure"
    )]
    pub message_activity_type: u32,
    pub party_id: Option<String>, //party id from rich presence event
}

pub enum MessageActivityType {
    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-object-message-activity-types"
    )]
    Join,
    Spectate,
    Listen,
    JoinRequest,
}

pub enum MessageType {
    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-object-message-types"
    )]
    // actually painful
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelNameChange,
    ChannelIconChange,
    ChannelPinnedMessage,
    UserJoin,
    GuildBoost,
    GuildBoostTier1,
    GuildBoostTier2,
    GuildBoostTier3,
    ChannelFollowAdd,
    GuildDiscoveryDisqualified,
    GuildDiscoveryRequalified,
    GuildDiscoveryGracePeriodInitialWarning,
    GuildDiscoveryGracePeriodFinalWarning,
    ThreadCreated,
    Reply,
    ChatInputCommand,
    ThreadStarterMessage,
    GuildInviteReminder,
    ContextMenuCommand,
    AutoModerationAction,
}
