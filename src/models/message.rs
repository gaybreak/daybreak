use enumflags2::bitflags;
use time::OffsetDateTime;

use super::{
    channel::{Channel, ChannelType},
    embed::Embed,
    emoji::{Sticker, StickerItem},
    interaction::{Component, MessageInteraction},
    member::Member,
    user::User,
    Id,
};
use crate::models::{application::Application, emoji::Emoji};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-object-message-structure"
)]
#[derive(Clone, Debug)]
pub struct Message {
    pub id: Id,
    pub channel_id: Id,
    pub author: Option<User>,
    pub content: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub edited_at: Option<OffsetDateTime>,
    pub tts: Option<bool>,
    pub mention_everyone: Option<bool>,
    pub mentions: Option<Vec<User>>,
    pub mention_roles: Option<Vec<Id>>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Option<Vec<Attachment>>,
    pub embeds: Option<Vec<Embed>>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    pub pinned: Option<bool>,
    pub webhook_id: Option<Id>,
    pub message_type: Option<MessageType>,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<Id>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<MessageFlags>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction: Option<MessageInteraction>,
    pub thread: Option<Channel>,
    pub components: Option<Vec<Component>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub stickers: Option<Vec<Sticker>>,
    pub position: Option<u32>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-create-message-create-extra-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway#message-update"
    )]
    pub guild_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-create-message-create-extra-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway#message-update"
    )]
    pub member: Option<Member>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-object-message-types"
)]
#[derive(Clone, Copy, Debug)]
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

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #message-object-message-activity-structure"
)]
#[derive(Clone, Debug)]
pub struct MessageActivity {
    pub message_activity_type: MessageActivityType,
    pub party_id: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-object-message-activity-types"
)]
#[derive(Clone, Copy, Debug)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[bitflags]
#[repr(u16)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-object-message-flags"
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageFlags {
    Crossposted = 1 << 0,
    IsCrosspost = 1 << 1,
    SuppressEmbeds = 1 << 2,
    SourceMessageDeleted = 1 << 3,
    Urgent = 1 << 4,
    HasThread = 1 << 5,
    Ephemeral = 1 << 6,
    Loading = 1 << 7,
    FailedToMentionSomeRolesInThread = 1 << 8,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#message-reference-object"
)]
#[derive(Clone, Copy, Debug)]
pub struct MessageReference {
    pub message_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub guild_id: Option<Id>,
    pub fail_if_not_exists: Option<bool>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #channel-mention-object-channel-mention-structure"
)]
#[derive(Clone, Debug)]
pub struct ChannelMention {
    pub id: Id,
    pub guild_id: Id,
    pub kind: ChannelType,
    pub name: String,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #allowed-mentions-object-allowed-mentions-structure"
)]
#[derive(Clone, Debug)]
pub struct AllowedMentions {
    pub parse: Vec<String>,
    pub roles: Vec<Id>,
    pub users: Vec<Id>,
    pub replied_user: bool,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#attachment-object-attachment-structure"
)]
#[derive(Clone, Debug)]
pub struct Attachment {
    pub id: Id,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u32,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub ephemeral: Option<bool>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#reaction-object-reaction-structure"
)]
#[derive(Clone, Debug)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub emoji: Emoji,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-add-message-reaction-add-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-message-reaction-remove-event-fields"
    )]
    pub user_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-add-message-reaction-add-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-message-reaction-remove-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-emoji-message-reaction-remove-emoji-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-emoji-message-reaction-remove-emoji-event-fields"
    )]
    pub channel_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-add-message-reaction-add-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-message-reaction-remove-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-emoji-message-reaction-remove-emoji-event-fields"
    )]
    pub message_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-add-message-reaction-add-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-message-reaction-remove-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-emoji-message-reaction-remove-emoji-event-fields"
    )]
    pub guild_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-add-message-reaction-add-event-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #message-reaction-remove-message-reaction-remove-event-fields"
    )]
    pub member: Option<Member>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #message-reaction-remove-all-message-reaction-remove-all-event-fields"
)]
#[derive(Clone, Copy, Debug)]
pub struct PrunedReactions {
    pub channel_id: Id,
    pub message_id: Id,
    pub guild_id: Option<Id>,
}
