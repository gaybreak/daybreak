use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::application::Application;
use crate::model::guild::UnavailableGuild;

use super::{
    presence::UpdatedPresence, user::User, Id,
};

#[bitflags]
#[repr(u32)]
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#list-of-intents")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum Intents {
    Guilds = 1 << 0,
    GuildMembers = 1 << 1,
    GuildBans = 1 << 2,
    GuildEmojisAndStickers = 1 << 3,
    GuildIntegrations = 1 << 4,
    GuildWebhooks = 1 << 5,
    GuildInvites = 1 << 6,
    GuildVoiceStates = 1 << 7,
    GuildPresences = 1 << 8,
    GuildMessages = 1 << 9,
    GuildMessageReactions = 1 << 10,
    GuildMessageTyping = 1 << 11,
    DirectMessages = 1 << 12,
    DirectMessageReactions = 1 << 13,
    DirectMessageTyping = 1 << 14,
    MessageContent = 1 << 15,
    GuildScheduledEvents = 1 << 16,
    AutoModerationConfiguration = 1 << 20,
    AutoModerationExecution = 1 << 21,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#payloads-gateway-payload-structure"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub op: GatewayOpcode,
    pub d: Option<String>,
    pub s: Option<u32>,
    pub t: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes"
)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum GatewayOpcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#identify-identify-structure"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identify {
    pub token: String,
    pub properties: IdentifyConnection,
    pub compress: Option<bool>,
    pub large_threshold: Option<u8>,
    pub shard: Option<(u16, u16)>,
    pub presence: Option<UpdatedPresence>,
    pub intents: Intents,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#identify-identify-connection-properties"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentifyConnection {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#resume-resume-structure")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resume {
    pub token: String,
    pub session_id: String,
    pub seq: u32,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #request-guild-members-guild-request-members-structure"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestGuildMembers {
    pub guild_id: Id,
    pub query: Option<String>,
    pub limit: Option<u16>,
    pub presences: Option<bool>,
    pub user_ids: Option<Vec<Id>>,
    pub nonce: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#get-gateway-bot-json-response"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotGateway {
    pub url: Option<String>,
    pub shards: u16,
    pub session_start_limit: SessionStartLimit,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #session-start-limit-object-session-start-limit-structure"
)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u32,
    pub max_concurrency: u32,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway\
        #hello-hello-structure"
)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Hello {
    pub heartbeat_interval: u32,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway\
        #ready-ready-event-fields"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ready {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<(u16, u16)>,
    pub application: Application,
}

