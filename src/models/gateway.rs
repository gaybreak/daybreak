use super::{application::Application, guild::UnavailableGuild, user::User};
use crate::models::presence::Activity;
use crate::models::Id;

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#hello-hello-structure")]
#[derive(Clone, Copy, Debug)]
pub struct Hello {
    pub heartbeat_interval: u32,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#ready-ready-event-fields")]
#[derive(Clone, Debug)]
pub struct Ready {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<(u16, u16)>,
    pub application: Application,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway\
#payloads-gateway-payload-structure")]
#[derive(Clone, Debug)]
pub struct Payload<T> {
    pub op: GatewayOpcode,
    pub d: Option<T>,
    pub s: Option<u32>,
    // Event name for the payload
    // None if opcode is not 0 (Dispatch)
    pub t: Option<String>,
}

// Outgoing payloads

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#identify")]
#[derive(Clone, Debug)]
pub struct Identify {
    pub token: String,
    pub properties: IdentifyProperties,
    pub compress: Option<bool>,
    pub large_threshold: Option<u8>,
    pub shard: Option<(u16, u16)>,
    pub presence: UpdatePresence,
    pub intents: u32,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#resume")]
#[derive(Clone, Debug)]
pub struct Resume {
    pub token: String,
    pub session_id: String,
    pub seq: u32,
}

// A heartbeat is just a base payload with only the opcode and data,
// which is the last sequence number of the last payload. If not yet received, None.
//
//
// #[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#heartbeat")]
// #[derive(Clone, Copy, Debug)]
// pub struct Heartbeat {
//     pub d: Option<u16>
// }

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#request-guild-members")]
#[derive(Clone, Debug)]
pub struct RequestGuildMembers {
    pub guild_id: Id,
    pub query: Option<String>,
    pub limit: u16,
    pub presences: Option<bool>,
    pub user_ids: Option<Vec<Id>>,
    // Nonce can only be up to 32 bytes
    pub nonce: Option<String>,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#update-voice-state")]
#[derive(Clone, Copy, Debug)]
pub struct UpdateVoiceState {
    pub guild_id: Id,
    pub channel_id: Option<Id>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#update-presence")]
#[derive(Clone, Debug)]
pub struct UpdatePresence {
    pub since: Option<u16>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: bool,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway\
#identify-identify-connection-properties")]
#[derive(Clone, Debug)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/opcodes-and-status-codes\
#gateway-gateway-opcodes")]
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum VoiceOpcode {
    Identify = 0,
    SelectProtocol = 1,
    Ready = 2,
    Heartbeat = 3,
    SessionDescription = 4,
    Speaking = 5,
    HeartbeatACK = 6,
    Resume = 7,
    Hello = 8,
    Resumed = 9,
    ClientDisconnect = 13,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway\
#session-start-limit-object-session-start-limit-structure")]
#[derive(Clone, Copy, Debug)]
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u32,
    pub max_concurrency: u32,
}
