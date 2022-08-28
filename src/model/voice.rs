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