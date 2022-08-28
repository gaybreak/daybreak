use time::OffsetDateTime;

use super::{member::Member, Id};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/voice#voice-state-object-voice-state-structure"
)]
#[derive(Clone, Debug)]
pub struct VoiceState {
    pub guild_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub user_id: Id,
    pub member: Option<Member>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<OffsetDateTime>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #voice-server-update-voice-server-update-event-fields"
)]
#[derive(Clone, Debug)]
pub struct VoiceServer {
    pub token: String,
    pub guild_id: Id,
    pub endpoint: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #update-voice-state-gateway-voice-state-update-structure"
)]
#[derive(Clone, Copy, Debug)]
pub struct UpdatedVoiceState {
    pub guild_id: Id,
    pub channel_id: Option<Id>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-opcodes"
)]
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
